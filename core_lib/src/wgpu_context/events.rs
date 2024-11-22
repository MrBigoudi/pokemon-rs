use log::{error, warn};
use winit::dpi::PhysicalSize;

use crate::{scene::rendering::frame::FrameData, utils::debug::ErrorCode};

use super::state::State;

impl State {
    pub fn on_resize(&self, new_size: PhysicalSize<u32>) -> Result<(), ErrorCode> {
        if new_size.width == 0 || new_size.height == 0 {
            warn!("Can't have a surface size that is zero");
            return Ok(());
        }
        {
            let mut config = self.config.lock().unwrap();
            config.width = new_size.width;
            config.height = new_size.height;
        }
        {
            let mut size = self.size.lock().unwrap();
            size.width = new_size.width;
            size.height = new_size.height;
        }
        self.surface
            .configure(&self.device, &self.config.lock().unwrap());
        Ok(())
    }

    /// Run at the begining of the rendering
    /// Initialize the framebuffer and the cammand buffer
    pub fn on_begin_render(&self) -> Result<FrameData, ErrorCode> {
        // Create a framebuffer
        let frame_buffer = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(err) => {
                error!("Failed to get the current surface texture: {:?}", err);
                return Err(ErrorCode::Wgpu);
            }
        };

        // Create a command buffer
        let command_buffer = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Command Encoder"),
            });

        Ok(FrameData {
            frame_buffer,
            command_buffer,
        })
    }

    /// Run at the end of the rendering
    /// Present the framebuffer to the sceen
    pub fn on_end_render(&self, frame_data: FrameData) {
        // Submit to the queue
        self.queue
            .submit(std::iter::once(frame_data.command_buffer.finish()));
        // Present to the screen
        frame_data.frame_buffer.present();
    }
}
