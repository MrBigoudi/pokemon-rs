use std::collections::HashMap;

use core_lib::{
    scene::{
        geometry::vertex::{RECTANGLE_INDICES, RECTANGLE_VERTICES},
        rendering::{
            frame::FrameData, graphics_pipelines::graphics_default::DefaultGraphicsPipeline,
        },
    },
    utils::{debug::ErrorCode, time::Duration},
    wgpu_context::global::get_global_wgpu_state,
    window::key_map::{Key, KeyState},
    DeviceExt,
};

use crate::states::state::{GameState, GameStateType};

pub struct GameStateTest {
    pub graphics_pipeline: DefaultGraphicsPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl GameStateTest {
    fn init_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TriangleVertexBuffer"),
            contents: bytemuck::cast_slice(RECTANGLE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        vertex_buffer
    }

    fn init_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TriangleIndexBuffer"),
            contents: bytemuck::cast_slice(RECTANGLE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        vertex_buffer
    }

    fn init_graphics_pipeline() -> DefaultGraphicsPipeline {
        pollster::block_on(DefaultGraphicsPipeline::new()).unwrap()
    }
}

impl Default for GameStateTest {
    fn default() -> Self {
        let graphics_pipeline = Self::init_graphics_pipeline();
        let device = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .device;
        let vertex_buffer = Self::init_vertex_buffer(device);
        let index_buffer = Self::init_index_buffer(device);
        Self {
            graphics_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}

impl GameState for GameStateTest {
    fn get_type(&self) -> GameStateType {
        GameStateType::Test
    }

    fn on_update(&mut self, _keys: &HashMap<Key, KeyState>, _delta_time: &Duration) {}

    fn on_exit(&mut self) {}

    fn on_enter(&mut self) {}

    fn on_keyboard_input(&mut self, _cur_keys: &HashMap<Key, KeyState>, _old_keys: &HashMap<Key, KeyState>, _new_key: &Key, _new_key_state: &KeyState) {}

    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        let output = &frame_data.frame_buffer;
        // Create a render pass
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder = &mut frame_data.command_buffer;
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let render_pipeline = &self.graphics_pipeline.base.render_pipeline;
        render_pass.set_pipeline(render_pipeline);

        // Diffuse texture bind group
        let bind_group_0 = &self.graphics_pipeline.base.bind_groups[0];
        render_pass.set_bind_group(0, bind_group_0, &[]);

        // Camera UBO bind group
        let bind_group_1 = &self.graphics_pipeline.base.bind_groups[1];
        render_pass.set_bind_group(1, bind_group_1, &[]);

        // Send vertices and indices
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // .. to use the entire buffer
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // .. to use the entire buffer

        let num_indices = RECTANGLE_INDICES.len() as u32;
        render_pass.draw_indexed(0..num_indices, 0, 0..1);

        Ok(())
    }

    fn on_resize(&mut self, _new_width: f32, _new_height: f32) {}
}
