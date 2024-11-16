use log::{error, warn};
use winit::{dpi::PhysicalSize, event::WindowEvent};

use crate::application::utils::{debug::ErrorCode, time::Duration};

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

    pub fn on_input(&self, _event: &WindowEvent) -> Result<(), ErrorCode> {
        // TODO:
        Ok(())
    }

    pub fn on_update(&self, _delta_time: &Duration) -> Result<(), ErrorCode> {
        // TODO:
        Ok(())
    }

    pub fn on_render(&self) -> Result<(), ErrorCode> {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(err) => {
                error!("Failed to get the current surface texture: {:?}", err);
                return Err(ErrorCode::Wgpu);
            }
        };

        // Create a command buffer
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Command Encoder"),
                });

        // Create a render pass
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // .. to use the entire buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // .. to use the entire buffer

            let num_indices = crate::scene::geometry::vertex::TRIANGLE_INDICES.len() as u32;
            render_pass.draw_indexed(0..num_indices, 0, 0..1); // 3 vertices, 1 instance
        }

        // Submit to the queue
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }
}
