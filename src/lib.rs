use crate::{GIN::Builders::state_builder::StateBuilderParticles};
use Legion::{ForceFields::SIN::Elements, Topology::atom::Atom};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, self},
    window::{WindowBuilder, Window},
};

// all this is for the macro SI types for now.
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Deref};
use std::convert::From;
use num_traits::float::FloatCore;
#[macro_use]
extern crate decay_si_derive;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use crate::GIN::{state::{State}, Builders::state_builder::StateBuilder};
// use crate::GIN::State;
// mod legion;
use log::{debug, error, log_enabled, info, Level};

mod GIN;

// #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]

static mut HEIGHT: u32 = 200;
static mut WIDTH: u32 = 200;

// pub trait DerefsToFloat {
//     type f32;
// }

#[derive(SITypes, SIDeref, PartialEq, Debug, Copy, Clone)]
struct Meter(f32);

// impl DerefsToFloat

// static mut STATE: &State;
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub struct DecayExport {
    state: State<Elements, f32, Atom<Elements, f32, Vec<f32>>, Vec<f32>>,
}

impl DecayExport {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                // console_log::init_with_level(log::Level::Warn).expect("error initializing logger");
            } else {
                env_logger::init();
            }
        }
        
        let window = WindowBuilder::new().build(event_loop).unwrap();
    
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(200, 200));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("decay")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
    
        // Adding in the state!
        // let state = State::new(window).await;
        let state = StateBuilder::<Elements, f32, Atom<Elements, f32, Vec<f32>>, Vec<f32>>::new(window)
            .size()
            .instance()
            .surface()
            .adapter().await
            .device_queue().await
            .config()
            .render_pipeline()
            .vertex_buffer()
            .index_buffer()
            .num_indices()
            .num_vertices()
            .camera()
            .camera_uniform()
            .camera_buffer()
            .camera_bind_group()
            .camera_controller()
            .time()
            .time_uniform()
            .time_buffer()
            .time_bind_group()
            .instances()
            .instance_buffer()
            .rng()
            .particles()
            .dimensions()
            .cell()
            .sin()
            .cell_set_particles(Elements::H(0))
            .integrator()
            .build();

        let mut _dt = 0.0;

        Self {
            state,
        }
    } 

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub unsafe fn resize(width: u32, height: u32)
{
    HEIGHT = height;
    WIDTH = width;
}    

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub async fn run() {
    let event_loop = EventLoop::new();
    let mut decay = DecayExport::new(&event_loop).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == decay.state.window().id() => {
            decay.state.update();
            unsafe {
                if (decay.state.size.height != HEIGHT || decay.state.size.width != WIDTH)
                {
                    // decay.state.size.height = HEIGHT;
                    // decay.state.size.width = WIDTH;
                    decay.state.resize(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT))
                }

            }
            // dt += 0.02;
            // web_sys::console::log_2(&"%s : Hello World".into());
            match decay.state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => decay.state.resize(decay.state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            decay.state.window().request_redraw();
        }
        
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == decay.state.window.id() => if !decay.state.input(event) {
            match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { 
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Plus | VirtualKeyCode::Equals),
                    ..
                },
                ..
             } => {
                let x = decay.state.size.height+10;
                let y = decay.state.size.width+10;
                decay.state.resize(winit::dpi::PhysicalSize::new(x, y) )
            }
            WindowEvent::KeyboardInput { 
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Minus),
                    ..
                },
                ..
             } => {
                let x = decay.state.size.height-10;
                let y = decay.state.size.width-10;
                decay.state.resize(winit::dpi::PhysicalSize::new(x, y) )
            }
            WindowEvent::Resized(physical_size) => {
                decay.state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                decay.state.resize(**new_inner_size);
            }
            _ => {}
        }
    }
        _ => {}
    });
}