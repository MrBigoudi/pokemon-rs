use core::debug::ErrorCode;
use std::{collections::HashMap, time::{Duration, Instant}};

use log::{error, info};
use parameters::ApplicationParameters;
use window::{init::WindowContext, key_map::{Key, KeyState}};
use winit::{dpi::LogicalPosition, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::Window};

mod window;
mod core;

mod parameters;

pub struct Application {
    window:  Option<Window>,
    last_frame: Instant,
    delta_time: Duration,
    keys: HashMap<Key, KeyState>,
    mouse_position: LogicalPosition<f64>,
}

impl Default for Application {
    fn default() -> Self {
        Self { 
            window: Default::default(), 
            last_frame: Instant::now(), 
            delta_time: Default::default(), 
            keys: Default::default(), 
            mouse_position: Default::default() 
        }
    }
}

impl Application {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErrorCode> {
        info!("Initializing parameters...");
        let parameters = ApplicationParameters::default();

        info!("Initializing the window...");
        let window = match WindowContext::init(&parameters, event_loop){
            Ok(window) => window,
            Err(err) => {
                error!("Failed to initialize the application's window: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

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
