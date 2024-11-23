use core_lib::{
    utils::{debug::ErrorCode, time::Instant}, window::key_map::{Key, KeyState}
};
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
        let mut frame_data = match self.wgpu_state.on_begin_render() {
            Ok(frame) => frame,
            Err(err) => {
                error!(
                    "Failed to initialize the reder pass on the wgpu state: {:?}",
                    err
                );
                return Err(ErrorCode::Wgpu);
            }
        };

        self.game_states.on_render(&mut frame_data)?;

        self.wgpu_state.on_end_render(frame_data);
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

        let new_width = new_size.width as f32;
        let new_height = new_size.height as f32;

        // Update game states
        self.game_states.on_resize(new_width, new_height);

        Ok(())
    }

    pub fn on_update(&mut self) -> Result<(), ErrorCode> {
        // Update delta time and cap the frame rate
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        if self.delta_time < self.target_frame_time {
            Instant::sleep(self.target_frame_time - self.delta_time);
        }

        // Update game state
        if let Err(err) = self.game_states.on_update(&self.keys, &self.delta_time){
            error!("Failed to update the game states: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }

    pub fn on_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) {
        if let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state,
            ..
        } = event
        {
            if let Some(key) = Key::from_winit(key_code) {
                let state = KeyState::from_winit(state);
                // Update global keys
                let _ = self.keys.insert(key, state);
                // Update game states
                self.game_states.on_keyboard_input(&self.keys, &self.last_keys, &key, &state);
            }
        }
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
