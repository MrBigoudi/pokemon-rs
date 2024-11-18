use log::{error, warn};
use winit::{dpi::PhysicalSize, event::WindowEvent};

use crate::application::utils::{debug::ErrorCode, time::Duration};

use super::{pipelines::graphics::GraphicsPipeline, state::State};

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

    pub fn on_render(&self, default_graphics_pipeline: &crate::scene::rendering::graphics_pipelines::graphics_default::DefaultGraphicsPipeline) -> Result<(), ErrorCode> {
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

            let render_pipeline = &default_graphics_pipeline.get_base().render_pipeline;
            render_pass.set_pipeline(render_pipeline);

            // Diffuse texture bind group
            let bind_group_0 = &default_graphics_pipeline.get_base().bind_groups[0];
            render_pass.set_bind_group(0, bind_group_0, &[]);

            // Camera UBO bind group
            let bind_group_1 = &default_graphics_pipeline.get_base().bind_groups[1];
            render_pass.set_bind_group(1, bind_group_1, &[]);

            // Send vertices and indices
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // .. to use the entire buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // .. to use the entire buffer

            let num_indices = crate::scene::geometry::vertex::RECTANGLE_INDICES.len() as u32;
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        // Submit to the queue
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }
}
