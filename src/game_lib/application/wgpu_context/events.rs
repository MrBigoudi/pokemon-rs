use winit::{dpi::PhysicalSize, event::WindowEvent};

use crate::application::core::debug::ErrorCode;

use super::state::State;

impl State {
    pub fn on_resize(&mut self, _new_size: PhysicalSize<u32>) -> Result<(), ErrorCode> {
        todo!()
    }

    pub fn on_input(&mut self, _event: &WindowEvent) -> Result<(), ErrorCode> {
        todo!()
    }

    pub fn on_update(&mut self) -> Result<(), ErrorCode> {
        todo!()
    }

    pub fn on_render(&mut self) -> Result<(), ErrorCode> {
        todo!()
    }
}
