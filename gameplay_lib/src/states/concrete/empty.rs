use std::collections::HashMap;

use core_lib::{
    scene::rendering::frame::FrameData,
    utils::{debug::ErrorCode, time::Duration},
    window::key_map::{Key, KeyState},
};

use crate::states::state::{GameState, GameStateType};

/// The default game state
pub struct GameStateEmpty;

impl GameState for GameStateEmpty {
    fn get_type(&self) -> GameStateType {
        GameStateType::Empty
    }

    fn on_update(&mut self, _keys: &HashMap<Key, KeyState>, _delta_time: &Duration) {}

    fn on_exit(&mut self) {}

    fn on_enter(&mut self) {}

    fn on_keyboard_input(
        &mut self,
        _cur_keys: &HashMap<Key, KeyState>,
        _old_keys: &HashMap<Key, KeyState>,
        _new_key: &Key,
        _new_key_state: &KeyState,
    ) {
    }

    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        // Draw the background
        // Needed to get the correct image format
        let output = &frame_data.frame_buffer;
        // Create a render pass
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder = &mut frame_data.command_buffer;
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
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

        Ok(())
    }

    fn on_resize(&mut self, _new_width: f32, _new_height: f32) {}

    fn should_be_swapped(&self) -> bool {
        false
    }

    fn should_be_removed(&self) -> bool {
        false
    }
}
