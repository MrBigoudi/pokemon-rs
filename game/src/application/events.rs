use common_lib::{debug::ErrorCode, time::Instant};
use core_lib::window::key_map::{Key, KeyState};
use log::error;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceId, KeyEvent},
    keyboard::PhysicalKey,
};

use crate::application::app::Application;


impl Application {
    pub fn on_exit(&mut self) -> Result<(), ErrorCode> {
        Ok(())
    }

    pub fn on_render(&mut self) -> Result<(), ErrorCode> {
        let mut new_frame = match self.wgpu_state.on_begin_render() {
            Ok(frame) => frame,
            Err(err) => {
                error!(
                    "Failed to initialize the reder pass on the wgpu state: {:?}",
                    err
                );
                return Err(ErrorCode::Wgpu);
            },
        };

        // TODO: add state render functions
        self.wgpu_state.on_render(&mut new_frame, &self.default_graphics_pipeline);

        self.wgpu_state.on_end_render(new_frame);
        Ok(())
    }

    pub fn on_resize(&mut self, new_size: PhysicalSize<u32>) -> Result<(), ErrorCode> {
        if let Err(err) = self.wgpu_state.on_resize(new_size) {
            error!(
                "Failed to handle a resize event on the wgpu state: {:?}",
                err
            );
            return Err(ErrorCode::Wgpu);
        }

        Ok(())
    }

    pub fn on_update(&mut self) -> Result<(), ErrorCode> {
        // Update delta time
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        // Update state
        if let Err(err) = self.wgpu_state.on_update(&self.delta_time) {
            error!(
                "Failed to handle an update event on the wgpu state: {:?}",
                err
            );
            return Err(ErrorCode::Wgpu);
        }
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
