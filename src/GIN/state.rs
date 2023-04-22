use std::{collections::HashMap};
use std::marker::PhantomData;
use num_traits::Float;
extern crate decay_si_derive;

use cgmath::{num_traits::ToPrimitive, prelude::*};
use num_traits::float::FloatCore;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::{WindowEvent},
    window::Window,
};

use Legion::{
    Dynamics::integrator::{Integrator, Leapfrog},
    ForceFields::SIN::{self, Elements},
    Topology::atom::{Atom, Atomic},
    Topology::particle::{HasPhysics},
    Topology::cell::{ContainsParticles, Cell},
};

use crate::GIN::{camera, instance, time};

const ROTATION_SPEED: f32 = 2.0 * std::f32::consts::PI / 60.0;

pub struct State <EleT, NumT, ParT, VecT>
where
    ParT: Atomic<EleT, NumT, VecT>,
    VecT: IntoIterator<Item = NumT>,
    NumT: FloatCore
{
    pub(crate) phantom: PhantomData<VecT>,
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    // add in the pipeline!
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) num_vertices: u32,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) camera: camera::Camera,
    pub(crate) camera_controller: camera::CameraController,
    pub(crate) camera_uniform: camera::CameraUniform,
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) time: [f32; 4],
    pub(crate) time_buffer: wgpu::Buffer,
    pub(crate) time_bind_group: wgpu::BindGroup,
    pub(crate) time_uniform: time::TimeUniform,
    pub(crate) instances: Vec<instance::Instance>,
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) rng: rand::rngs::ThreadRng,
    pub(crate) cell: Cell<ParT, NumT>,
    pub(crate) dimensions: u32,
    pub(crate) integrator: Leapfrog<NumT>,
    pub(crate) sin: SIN::SIN<EleT>,
}

impl State<Elements, f64, Atom<Elements, f64, Vec<f64>>, Vec<f64>> {
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
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // update the dynamics!  DO NOT WRITE DURING THIS TIME.
        // let newWorld: HashMap::<String, Box<dyn Atomic>> = HashMap::new();
        let mut accVec = HashMap::<String, Vec<f64>>::new();
        for (name, _) in self.cell.get_particles() {
            accVec.insert(
                name.clone(),
                self.integrator
                    .calculate_forces(name.clone(), &self.cell, &self.sin),
            );
        }

        // NOW we want to write.  So we use a different method: get mut particles!
        for (atom, acc) in accVec.iter_mut() {
            let particle = self.cell.get_mut_particles().get_mut(atom);
            match particle {
                Some(a) => {
                    let (pos, vel, acc) = self.integrator.integrate(a, acc.to_vec());
                    a.set_position(pos);
                    a.set_velocity(vel);
                    // a.set_acceleration(acc;)
                }
                None => (),
            }
        }

        for instance in &mut self.instances {
            let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(ROTATION_SPEED));
            let current = instance.rotation;
            instance.rotation = amount * current;
            let atom_pos = self
                .cell
                .get_particles()
                .get(&instance.id.clone().unwrap())
                .unwrap()
                .get_position();
            instance.position = instance.original_position;
            instance.position.x += atom_pos.get(0).unwrap().to_f32().unwrap();

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
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
