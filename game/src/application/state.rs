use core_lib::utils::config::ApplicationParameters;

use log::warn;
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use super::app::Application;

pub enum ApplicationState {
    Uninitialized { parameters: ApplicationParameters },
    Initialized(Application),
}

impl ApplicationState {
    fn init(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        take_mut::take(self, |state| match state {
            ApplicationState::Initialized(_) => {
                panic!("Failed to initialize the application state")
            }
            ApplicationState::Uninitialized { parameters } => Self::Initialized(
                Application::new(event_loop, parameters)
                    .expect("Failed to initialize the application"),
            ),
        });
    }
}

impl ApplicationHandler for ApplicationState {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        match self {
            ApplicationState::Uninitialized { .. } => {
                warn!("The application is not initialized correctly...")
            }
            ApplicationState::Initialized(app) => app.window.request_redraw(),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let ApplicationState::Uninitialized { .. } = self {
            self.init(event_loop)
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        match self {
            ApplicationState::Uninitialized { .. } => {
                warn!("The application is not initialized correctly...")
            }
            ApplicationState::Initialized(app) => {
                app.on_exit().expect("Failed to handle exiting event");
            }
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        match self {
            ApplicationState::Uninitialized { .. } => {
                warn!("The application is not initialized correctly...")
            }
            ApplicationState::Initialized(app) => {
                app.on_update().expect("Failed to handle update event");
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let app = match self {
            ApplicationState::Uninitialized { .. } => {
                warn!("The application is not initialized correctly...");
                return;
            }
            ApplicationState::Initialized(app) => app,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                app.on_resize(new_size)
                    .expect("Failed to handle resizing event");
            }
            WindowEvent::RedrawRequested => {
                app.on_render().expect("Failed to handle redrawing event");
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                app.on_keyboard_input(device_id, event, is_synthetic);
            }

            #[cfg(not(target_arch = "wasm32"))]
            WindowEvent::CursorMoved {
                position,
                device_id,
            } => {
                let new_position = position.to_logical(app.window.scale_factor());
                app.on_mouse_moved(device_id, new_position)
                    .expect("Failed to handle mouse moved event");
            }
            _ => (),
        }
    }
}
