use std::{collections::HashMap, ops::Deref, borrow::BorrowMut};

use winit::{window::Window, event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};
use wgpu::util::DeviceExt;
use log::{debug, error, log_enabled, info, Level};
use cgmath::{prelude::*, num_traits::ToPrimitive};
use rand::{Rng, prelude::Distribution};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


use crate::legion::{topology::particle::{HasPhysics, self, IsSpatial}, topology::atom::{Atom, IsAtomic, Connected}, dynamics::integrator::{Leapfrog, Integrator}, topology::spaceTime::{ContainsParticles, self, SpaceTime}, sin::ff::{self, ForceField, Elements}};

use super::{camera, time, vertex, primitives, instance};

const ROTATION_SPEED: f32 = 2.0 * std::f32::consts::PI / 60.0;

pub(crate) struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    // add in the pipeline!
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer, 
    num_indices: u32,
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    time: [f32; 4],
    time_buffer: wgpu::Buffer,
    time_bind_group: wgpu::BindGroup,
    time_uniform: time::TimeUniform,
    instances: Vec<instance::Instance>,
    instance_buffer: wgpu::Buffer,
    rng: rand::rngs::ThreadRng,
    // particles: Option<HashMap<String, Box<dyn IsAtomic>>>,
    spaceTime: SpaceTime<Atom<Elements, f64, Vec<f64>>, f64>,
    dimensions: u32,
    integrator: Leapfrog<f64>,
    sin: ff::SIN<Elements>
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
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
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // load in the shaders!
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        // camera render stuff
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        // camera render stuff
        let time_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("time_bind_group_layout"),
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &time_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    vertex::Vertex::desc(),
                    instance::InstanceRaw::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
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
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(primitives::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(primitives::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = primitives::INDICES.len() as u32;
        let num_vertices = primitives::VERTICES.len() as u32;

        let camera = camera::Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = camera::CameraUniform::new();
            camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        let camera_controller = camera::CameraController::new(0.2);

        let time = [0.0,0.0,0.0,0.0];

        let mut time_uniform = time::TimeUniform::new();
            time_uniform.update_time(time);

        let time_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Time Buffer"),
                contents: bytemuck::cast_slice(&[time_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &time_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: time_buffer.as_entire_binding(),
                }
            ],
            label: Some("time_bind_group"),
        });

        let mut instances = (0..instance::NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..instance::NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - instance::INSTANCE_DISPLACEMENT;
                let original_position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - instance::INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can effect scale if they're not created correctly
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                instance::Instance {
                    position, original_position, rotation, id: None
                }
            })
        }).collect::<Vec<_>>();
        let instance_data = instances.iter().map(instance::Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let rng = rand::thread_rng();
        let mut particles = HashMap::<String, Atom<Elements, f64, Vec<f64>>>::new();
        let dimensions: u32 = 3;

        let mut spaceTime = SpaceTime::<Atom<Elements, f64, Vec<f64>>, f64>::new();
        // let's just make some atoms!
        // let's make them use some of the instance things.
        let sin = ff::SIN::<Elements> { description: "SIN".to_string(), particle_type: Vec::new() };
        
        let mut priorAtom = "".to_string();
        // Add in an atom for each triangle!  Fake a bond, make it work designers!
        let mut allAtoms = Vec::<String>::new();
        for instance in &mut instances {
            let mut atom = sin.atom(ff::Elements::H(0));
            atom.generate_spatial_coordinates(3);
            instance.id = Some(atom.id.clone());
            let pos = vec!(instance.position.x.to_f64().unwrap(), instance.position.y.to_f64().unwrap(), instance.position.z.to_f64().unwrap());
            atom.set_position(pos);
            let mut rng = rand::thread_rng();
            let sign: rand::distributions::Uniform<f64> = rand::distributions::Uniform::from(-1.0..1.1);
            let applyJitter = true;
            if applyJitter {
                let mut vel = vec![0.0; 3];
                for i in 0..3 {
                    vel[i] = (rng.gen_range(0.0..1000.0)) * sign.sample(&mut rng);
                }
                atom.set_velocity(vel);
            }
            if priorAtom != "".to_string() {
                atom.neighbors.push(priorAtom.clone());
            }
            priorAtom = atom.id.clone();
            allAtoms.push(atom.id.clone());
            particles.insert(atom.id.clone(), atom); // we clone/copy the string to avoid problems with lifetimes.
        }
        spaceTime.set_particles(particles);

        // just make a big ol chain.
        // for name in allAtoms.iter() {
        //     let particle = &mut spaceTime.get_mut_particles().get_mut(name);
        //     match particle {
        //         Some(a) => {
        //             a.set_neighbors(allAtoms.clone());
        //         }
        //         None => ()
        //     }
        // }

        let integrator = Leapfrog::new();

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            time,
            time_buffer,
            time_bind_group,
            time_uniform,
            instances,
            instance_buffer,
            rng,
            spaceTime,
            dimensions,
            integrator,
            sin
        }
    }

    pub fn integrator(&mut self) -> &Leapfrog<f64> {
        &self.integrator
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.window.set_inner_size(new_size);
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));

        // update the dynamics!  DO NOT WRITE DURING THIS TIME.
        // let newWorld: HashMap::<String, Box<dyn IsAtomic>> = HashMap::new();
        let mut accVec = HashMap::<String, Vec<f64>>::new();
        for (name, _) in self.spaceTime.get_particles() {
            accVec.insert(name.clone(), self.integrator.calculate_forces(name.clone(), &self.spaceTime, &self.sin));
        }

        // NOW we want to write.  So we use a different method: get mut particles!
        for (atom, acc) in accVec.iter_mut() {
            let particle = self.spaceTime.get_mut_particles().get_mut(atom);
            match particle {
                Some(a) => {
                    let (pos, vel, acc) = self.integrator.integrate(a, acc.to_vec());
                    a.set_position(pos);
                    a.set_velocity(vel);
                    // a.set_acceleration(acc;)
                }
                None => ()
            }
            
        }

        // TODO MOVE TIME
        // self.time[0] += 0.0002;
        // self.time[1] += 0.0002;
        // self.time[2] += 0.0002;
        // self.time[3] += 0.0002;
        // self.time_uniform.update_time(self.time);
        // self.queue.write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[self.time_uniform]));

        // this is from the challenge.rs; look how the instance position update and modification is done!
        // looks like we update the buffer; interesting!
        // start up some random jitter, just to test.
        for instance in &mut self.instances {
            let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(ROTATION_SPEED));
            let current = instance.rotation;
            instance.rotation = amount * current;
            let atom_pos = self.spaceTime.get_particles()
            .get(&instance.id.clone().unwrap())
            .unwrap()
            .get_position();
            // let atom_pos = world.get(&instance.id.clone().unwrap()).clone().unwrap().get_position();
            instance.position = instance.original_position;
            instance.position.x += atom_pos.get(0)
            .unwrap()
            .to_f32()
            .unwrap();
            
            
            instance.position.y += atom_pos.get(1).unwrap().to_f32().unwrap();
            instance.position.z += atom_pos.get(2).unwrap().to_f32().unwrap();
        }
        let instance_data = self
            .instances
            .iter()
            .map(instance::Instance::to_raw)
            .collect::<Vec<_>>();
    
        self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&instance_data),
            );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.003,
                            g: 0.804,
                            b: 0.996,
                            a: 1.00,
                        }),
                        store: true, // 1, 205, 254, 0.25
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline); // 2.
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.time_bind_group, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // instances!
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // UPDATED!
            // Make sure if you add new instances to the Vec, that you recreate the instance_buffer and as well as camera_bind_group, otherwise your new instances won't show up correctly.
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);

            // render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.
            
            // render_pass.draw(0..self.num_vertices, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}