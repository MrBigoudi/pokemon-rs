use std::collections::HashMap;

use core_lib::{
    scene::{rendering::frame::FrameData, text::{font::FontFamily, global::resize_global_font_system, text_instance::{TextInstance, TextParameters}}}, utils::{debug::ErrorCode, time::Duration}, window::key_map::{Key, KeyState}
};
use log::error;

use crate::states::state::{GameState, GameStateType};

/// The game state to render dialog
pub struct GameStateOverworldDialog {
    // For displaying text
    pub text_instance: TextInstance,
    // Other attributes
    pub should_be_swapped: bool,
}

impl GameStateOverworldDialog {
    pub fn new() -> Result<Self, ErrorCode> {
        let parameters = TextParameters {
            content: String::from("Press escape to leave this state"),
            font_size: 30.,
            line_height: 42.,
            font_family: FontFamily::SansSerif,
            font_color: [255, 255, 255, 255],
            bounds_left: 10.,
            bounds_top: 10.,
            bounds_width: 600,
            bounds_height: 160,
        };
        let text_instance = match TextInstance::new(parameters) {
            Ok(instance) => instance,
            Err(err) => {
                error!("Failed to create the text instance for the game state `{:?}': {:?}", GameStateType::OverworldDialog, err);
                return Err(ErrorCode::Unknown);
            }
        };

        Ok(Self {
            text_instance,
            should_be_swapped: false,
        })
    }
}

impl GameState for GameStateOverworldDialog {
    fn get_type(&self) -> GameStateType {
        GameStateType::OverworldDialog
    }

    fn on_update(&mut self, _keys: &HashMap<Key, KeyState>, _delta_time: &Duration) {}

    fn on_exit(&mut self) {
        self.should_be_swapped = true;
    }

    fn on_enter(&mut self) {
        self.should_be_swapped = false;
    }

    fn on_keyboard_input(&mut self, _cur_keys: &HashMap<Key, KeyState>, _old_keys: &HashMap<Key, KeyState>, new_key: &Key, new_key_state: &KeyState) {
        if *new_key_state == KeyState::Pressed && *new_key == Key::Escape {
            self.should_be_swapped = true;
        }
    }

    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        // Needed to get the correct image format
        let output = &frame_data.frame_buffer;
        // Create a render pass
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder = &mut frame_data.command_buffer;
        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        self.text_instance.render(render_pass)?;

        Ok(())
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        resize_global_font_system(new_width, new_height);
    }
    
    fn should_be_swapped(&self) -> bool {
        self.should_be_swapped
    }
    
    fn should_be_removed(&self) -> bool {
        false
    }
}
