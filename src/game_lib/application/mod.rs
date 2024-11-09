use core::{debug::ErrorCode, time::{Duration, Instant}};
use std::collections::HashMap;

use log::{error, info};
use parameters::ApplicationParameters;
use window::{
    init::WindowContext,
    key_map::{Key, KeyState},
};
use winit::{
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

pub mod core;
pub mod window;

pub mod parameters;

#[derive(Default)]
pub struct Application {
    window: Option<Window>,
    last_frame: Instant,
    delta_time: Duration,
    keys: HashMap<Key, KeyState>,
    #[cfg(not(target_arch = "wasm32"))]
    mouse_position: winit::dpi::LogicalPosition<f64>,
}

impl Application {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErrorCode> {
        info!("Initializing parameters...");
        let parameters = ApplicationParameters::default();

        info!("Initializing the window...");
        let window = match WindowContext::init(&parameters, event_loop) {
            Ok(window) => window,
            Err(err) => {
                error!("Failed to initialize the application's window: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            match web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let element_id = "wasm-example";
                    let dst = match doc.get_element_by_id(element_id) {
                        Some(dst) => dst,
                        None => {
                            error!("Failed to get the element with id `{}'", element_id);
                            return Some(Err(ErrorCode::Web));
                        }
                    };
                    let canvas = web_sys::Element::from(match window.canvas() {
                        Some(canvas) => canvas,
                        None => {
                            error!("Failed to get the window's canvas");
                            return Some(Err(ErrorCode::Winit));
                        }
                    });
                    if let Err(err) = dst.append_child(&canvas) {
                        error!("Failed to append a child to the document body: {:?}", err);
                        return Some(Err(ErrorCode::Web));
                    };
                    Some(Ok(()))
                }) {
                None => {
                    error!("Failed to append canvas to document body");
                    return Err(ErrorCode::Unknown);
                }
                Some(Err(err)) => {
                    error!("Failed to append canvas to document body: {:?}", err);
                    return Err(ErrorCode::Web);
                }
                _ => (),
            }
        }

        self.window = Some(window);
        Ok(())
    }

    pub fn run() -> Result<(), ErrorCode> {
        info!("Initializing the event loop...");
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::Winit);
            }
        };
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Application::default();
        if let Err(err) = event_loop.run_app(&mut app) {
            error!("An error occured during the main event loop: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), ErrorCode> {
        // Update delta time
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        Ok(())
    }
}
