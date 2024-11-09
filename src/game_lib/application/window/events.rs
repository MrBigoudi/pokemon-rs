use log::warn;
use winit::{
    application::ApplicationHandler,
    event::{DeviceId, KeyEvent, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::WindowId,
};

use crate::application::{core::debug::ErrorCode, Application};

use super::key_map::{Key, KeyState};

impl Application {
    pub fn on_exit(&mut self) -> Result<(), ErrorCode> {
        Ok(())
    }

    pub fn on_redraw(&mut self) -> Result<(), ErrorCode> {
        Ok(())
    }

    pub fn on_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) -> Result<(), ErrorCode> {
        if let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state,
            ..
        } = event
        {
            if let Some(key) = Key::from_winit(key_code) {
                let state = KeyState::from_winit(state);
                let _ = self.keys.insert(key, state);
            }
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn on_mouse_moved(
        &mut self,
        _device_id: DeviceId,
        new_position: winit::dpi::LogicalPosition<f64>,
    ) -> Result<(), ErrorCode> {
        self.mouse_position = new_position;
        Ok(())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Err(err) = self.init(event_loop) {
                panic!("Failed to initialize the winit window: {:?}", err);
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Err(err) = self.on_exit() {
            panic!("Failed to handle exiting event: {:?}", err);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        } else {
            warn!("The window is not initialized correctly...");
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        if let Err(err) = self.update() {
            panic!("Failed to update the application: {:?}", err);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.on_redraw() {
                    panic!("Failed to handle redrawing event: {:?}", err);
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let Err(err) = self.on_keyboard_input(device_id, event, is_synthetic) {
                    panic!("Failed to handle keyboard input event: {:?}", err);
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            WindowEvent::CursorMoved {
                position,
                device_id,
            } => {
                if let Some(window) = &self.window {
                    let new_position = position.to_logical(window.scale_factor());
                    if let Err(err) = self.on_mouse_moved(device_id, new_position) {
                        panic!("Failed to handle mouse moved event: {:?}", err);
                    }
                } else {
                    warn!("The window is not initialized correctly...");
                }
            }
            _ => (),
        }
    }
}
