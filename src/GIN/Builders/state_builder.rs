use num_traits::Float;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::Legion::ForceFields::SIN::ParticleGenerator;
use crate::GIN::state::State;
use cgmath::{num_traits::ToPrimitive, prelude::*};
use rand::{prelude::Distribution, Rng};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::Legion::{
    Dynamics::integrator::Leapfrog,
    ForceFields::SIN::{self, Elements},
    Topology::atom::{Atom, Atomic, Connected},
    Topology::particle::{HasPhysics, IsSpatial},
    Topology::cell::Cell,
};

use crate::GIN::{camera, instance, primitives, time, vertex};

pub trait StateBuilderParticles<EleT> {
    fn cell_set_particles(self, element: EleT) -> Self;
}

pub struct StateBuilder<EleT, NumT, ParT, VecT>
where
    ParT: Atomic<EleT, NumT, VecT>,
    VecT: IntoIterator<Item = NumT>,
    NumT: Float,
{
    phantom: PhantomData<VecT>,
    instance: Option<wgpu::Instance>,
    surface: Option<wgpu::Surface>,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
    size: Option<winit::dpi::PhysicalSize<u32>>,
    window: Window,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    num_vertices: Option<u32>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: Option<u32>,
    camera: Option<camera::Camera>,
    camera_controller: Option<camera::CameraController>,
    camera_uniform: Option<camera::CameraUniform>,
    camera_buffer: Option<wgpu::Buffer>,
    camera_bind_group: Option<wgpu::BindGroup>,
    camera_bind_group_layout: Option<wgpu::BindGroupLayout>,
    time: Option<[f32; 4]>,
    time_buffer: Option<wgpu::Buffer>,
    time_bind_group: Option<wgpu::BindGroup>,
    time_bind_group_layout: Option<wgpu::BindGroupLayout>,
    time_uniform: Option<time::TimeUniform>,
    instances: Option<Vec<instance::Instance>>,
    instance_buffer: Option<wgpu::Buffer>,
    rng: Option<rand::rngs::ThreadRng>,
    particles: Option<HashMap<String, ParT>>,
    cell: Option<Cell<ParT, NumT>>,
    dimensions: Option<u32>,
    integrator: Option<Leapfrog<NumT>>,
    sin: Option<SIN::SIN<EleT>>,
}

impl<EleT, NumT: Float, ParT, VecT> StateBuilder<EleT, NumT, ParT, VecT>
where
    ParT: Atomic<EleT, NumT, VecT> + IsSpatial,
    VecT: IntoIterator<Item = NumT>,
{
    pub fn new(window: Window) -> Self {
        Self {
            phantom: PhantomData::<VecT>,
            instance: None,
            surface: None,
            adapter: None,
            device: None,
            queue: None,
            config: None,
            size: None,
            window,
            render_pipeline: None,
            vertex_buffer: None,
            num_vertices: None,
            index_buffer: None,
            num_indices: None,
            camera: None,
            camera_controller: None,
            camera_uniform: None,
            camera_buffer: None,
            camera_bind_group: None,
            camera_bind_group_layout: None,
            time: None,
            time_buffer: None,
            time_bind_group: None,
            time_bind_group_layout: None,
            time_uniform: None,
            instances: None,
            instance_buffer: None,
            rng: None,
            particles: None,
            cell: None,
            dimensions: None,
            integrator: None,
            sin: None,
        }
    }

    pub fn size(mut self) -> Self {
        self.size = Some(self.window.inner_size());
        self
    }

    pub fn instance(mut self) -> Self {
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        self.instance = Some(wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        }));
        self
    }

    pub fn surface(mut self) -> Self {
        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        self.surface = unsafe {
            Some(
                self.instance
                    .as_ref()
                    .unwrap()
                    .create_surface(&self.window)
                    .unwrap(),
            )
        };
        self
    }

    pub async fn adapter(mut self) -> Self {
        self.adapter = self
            .instance
            .as_ref() // This is to ensure our call to unwrap doesn't consume it!
            .unwrap()
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&self.surface.as_ref().unwrap()), // POSSIBLY UNSAFE; look again after re-organizing! TODO
                force_fallback_adapter: false,
            })
            .await;
        self
    }

    pub async fn device_queue(mut self) -> Self {
        let (device, queue) = self
            .adapter
            .as_ref()
            .unwrap() // this moves it into the ownership.
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        self.device = Some(device);
        self.queue = Some(queue);
        self
    }

    pub fn config(mut self) -> Self {
        let surface_caps = self
            .surface
            .as_ref()
            .unwrap()
            .get_capabilities(&self.adapter.as_ref().unwrap());
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.size.as_ref().unwrap().width,
            height: self.size.as_ref().unwrap().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        self.surface
            .as_ref()
            .unwrap()
            .configure(&self.device.as_ref().unwrap(), &config);
        self.config = Some(config);
        self
    }

    pub fn render_pipeline(mut self) -> Self {
        // load in the shaders!
        let shader =
            self.device
                .as_ref()
                .unwrap()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("../../shaders/shader.wgsl").into(),
                    ),
                });

        // camera render stuff
        let camera_bind_group_layout = self.device.as_ref().unwrap().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            },
        );

        // camera render stuff
        let time_bind_group_layout = self.device.as_ref().unwrap().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("time_bind_group_layout"),
            },
        );

        let render_pipeline_layout =
            self.device
                .as_ref()
                .unwrap()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &time_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            self.device
                .as_ref()
                .unwrap()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main", // 1.
                        buffers: &[vertex::Vertex::desc(), instance::InstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        // 3.
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            // 4.
                            format: self.config.as_ref().unwrap().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None, // 1.
                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },
                    multiview: None, // 5.
                });

        self.render_pipeline = Some(render_pipeline);
        self.time_bind_group_layout = Some(time_bind_group_layout);
        self.camera_bind_group_layout = Some(camera_bind_group_layout);
        self
    }

    pub fn vertex_buffer(mut self) -> Self {
        let vertex_buffer =
            self.device
                .as_ref()
                .unwrap()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(primitives::VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        self.vertex_buffer = Some(vertex_buffer);
        self
    }

    pub fn index_buffer(mut self) -> Self {
        let index_buffer =
            self.device
                .as_ref()
                .unwrap()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(primitives::INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });
        self.index_buffer = Some(index_buffer);
        self
    }

    pub fn num_indices(mut self) -> Self {
        self.num_indices = Some(primitives::INDICES.len() as u32);
        self
    }

    pub fn num_vertices(mut self) -> Self {
        self.num_vertices = Some(primitives::VERTICES.len() as u32);
        self
    }

    pub fn camera(mut self) -> Self {
        let camera = camera::Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: self.config.as_ref().unwrap().width as f32
                / self.config.as_ref().unwrap().height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        self.camera = Some(camera);
        self
    }

    pub fn camera_uniform(mut self) -> Self {
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&self.camera.as_ref().unwrap());
        self.camera_uniform = Some(camera_uniform);
        self
    }

    pub fn camera_buffer(mut self) -> Self {
        let camera_buffer =
            self.device
                .as_ref()
                .unwrap()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[self.camera_uniform.unwrap()]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        self.camera_buffer = Some(camera_buffer);
        self
    }

    pub fn camera_bind_group(mut self) -> Self {
        let camera_bind_group =
            self.device
                .as_ref()
                .unwrap()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.camera_bind_group_layout.as_ref().unwrap(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.camera_buffer.as_ref().unwrap().as_entire_binding(),
                    }],
                    label: Some("camera_bind_group"),
                });
        self.camera_bind_group = Some(camera_bind_group);
        self
    }

    pub fn camera_controller(mut self) -> Self {
        self.camera_controller = Some(camera::CameraController::new(0.2));
        self
    }

    pub fn time(mut self) -> Self {
        self.time = Some([0.0, 0.0, 0.0, 0.0]);
        self
    }

    pub fn time_uniform(mut self) -> Self {
        let mut time_uniform = time::TimeUniform::new();
        time_uniform.update_time(self.time.unwrap());
        self.time_uniform = Some(time_uniform);
        self
    }

    pub fn time_buffer(mut self) -> Self {
        let time_buffer =
            self.device
                .as_ref()
                .unwrap()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Time Buffer"),
                    contents: bytemuck::cast_slice(&[self.time_uniform.unwrap()]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        self.time_buffer = Some(time_buffer);
        self
    }

    pub fn time_bind_group(mut self) -> Self {
        let time_bind_group =
            self.device
                .as_ref()
                .unwrap()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.time_bind_group_layout.as_ref().unwrap(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.time_buffer.as_ref().unwrap().as_entire_binding(),
                    }],
                    label: Some("time_bind_group"),
                });
        self.time_bind_group = Some(time_bind_group);
        self
    }

    pub fn instances(mut self) -> Self {
        let mut instances = (0..instance::NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..instance::NUM_INSTANCES_PER_ROW).map(move |x| {
                    let position = cgmath::Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    } - instance::INSTANCE_DISPLACEMENT;
                    let original_position = cgmath::Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    } - instance::INSTANCE_DISPLACEMENT;

                    let rotation = if position.is_zero() {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };

                    instance::Instance {
                        position,
                        original_position,
                        rotation,
                        id: None,
                    }
                })
            })
            .collect::<Vec<_>>();
        self.instances = Some(instances);
        self
    }

    pub fn instance_buffer(mut self) -> Self {
        let instance_data = self
            .instances
            .as_ref()
            .unwrap()
            .iter()
            .map(instance::Instance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer =
            self.device
                .as_ref()
                .unwrap()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        self.instance_buffer = Some(instance_buffer);
        self
    }

    pub fn rng(mut self) -> Self {
        self.rng = Some(rand::thread_rng()); // TODO: awful
        self
    }
    pub fn particles(mut self) -> Self {
        let mut particles = HashMap::<String, ParT>::new();
        self.particles = Some(particles);
        self
    }

    pub fn dimensions(mut self) -> Self {
        let dimensions: u32 = 3;
        self.dimensions = Some(dimensions);
        self
    }

    pub fn cell(mut self) -> Self {
        let cell = Cell::<ParT, NumT>::new();
        self.cell = Some(cell);
        self
    }

    pub fn sin(mut self) -> Self {
        // let's just make some atoms!
        // let's make them use some of the instance things.
        let sin = SIN::SIN::<EleT> {
            description: "SIN".to_string(),
            particle_type: Vec::new(),
        };
        self.sin = Some(sin);
        self
    }

    pub fn integrator(mut self) -> Self {
        let integrator = Leapfrog::<NumT>::new();
        self.integrator = Some(integrator);
        self
    }

    pub fn build(self) -> State<EleT, NumT, ParT, VecT> {
        State {
            phantom: self.phantom,
            window: self.window,
            surface: self.surface.unwrap(),
            device: self.device.unwrap(),
            queue: self.queue.unwrap(),
            config: self.config.unwrap(),
            size: self.size.unwrap(),
            render_pipeline: self.render_pipeline.unwrap(),
            vertex_buffer: self.vertex_buffer.unwrap(),
            num_vertices: self.num_vertices.unwrap(),
            index_buffer: self.index_buffer.unwrap(),
            num_indices: self.num_indices.unwrap(),
            camera: self.camera.unwrap(),
            camera_controller: self.camera_controller.unwrap(),
            camera_uniform: self.camera_uniform.unwrap(),
            camera_buffer: self.camera_buffer.unwrap(),
            camera_bind_group: self.camera_bind_group.unwrap(),
            time: self.time.unwrap(),
            time_buffer: self.time_buffer.unwrap(),
            time_bind_group: self.time_bind_group.unwrap(),
            time_uniform: self.time_uniform.unwrap(),
            instances: self.instances.unwrap(),
            instance_buffer: self.instance_buffer.unwrap(),
            rng: self.rng.unwrap(),
            cell: self.cell.unwrap(),
            dimensions: self.dimensions.unwrap(),
            integrator: self.integrator.unwrap(),
            sin: self.sin.unwrap(),
        }
    }
}

impl StateBuilderParticles<Elements>
    for StateBuilder<Elements, f64, Atom<Elements, f64, Vec<f64>>, Vec<f64>>
{
    fn cell_set_particles(mut self, element: Elements) -> Self {
        let mut priorAtom = "".to_string();
        // Add in an atom for each triangle!  Fake a bond, make it work designers!
        let mut allAtoms = Vec::<String>::new();
        let mut atomHashMap: HashMap<String, Atom<Elements, f64, Vec<f64>>> = HashMap::new();
        for instance in self.instances.as_mut().unwrap() {
            let mut atom: Atom<Elements, f64, Vec<f64>> = self
                .sin
                .as_ref()
                .unwrap()
                .generate_particle(element.clone());
            atom.generate_spatial_coordinates(3);
            instance.id = Some(atom.get_id().clone());
            let pos = vec![
                instance.position.x.to_f64().unwrap(),
                instance.position.y.to_f64().unwrap(),
                instance.position.z.to_f64().unwrap(),
            ];
            atom.set_position(pos);
            let mut rng = rand::thread_rng();
            let sign: rand::distributions::Uniform<f64> =
                rand::distributions::Uniform::from(-1.0..1.1);
            let applyJitter = true;
            if applyJitter {
                let mut vel = vec![0.0; 3];
                for i in 0..3 {
                    vel[i] = (rng.gen_range(0.0..1000.0) / 1000.0) * sign.sample(&mut rng);
                }
                atom.set_velocity(vel);
            }
            if priorAtom != "".to_string() {
                atom.neighbors.push(priorAtom.clone());
            }
            priorAtom = atom.id.clone();
            allAtoms.push(atom.id.clone());
            atomHashMap.insert(atom.id.clone(), atom);
        }
        self.cell.as_mut().unwrap().set_particles(atomHashMap);
        self
    }
}
