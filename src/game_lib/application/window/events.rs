use winit::{
    event::{DeviceId, KeyEvent},
    keyboard::PhysicalKey,
};

use crate::application::{app::Application, core::debug::ErrorCode};

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
