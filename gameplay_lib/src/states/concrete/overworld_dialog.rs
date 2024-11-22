use std::collections::HashMap;

use core_lib::{
    scene::rendering::frame::FrameData, utils::{debug::ErrorCode, time::Duration}, wgpu_context::global::get_global_wgpu_state, window::key_map::{Key, KeyState}
};
use log::error;


use crate::states::state::{GameState, GameStateType};

/// The game state to render dialog
pub struct GameStateOverworldDialog {
    pub font_system: glyphon::FontSystem,
    pub swash_cache: glyphon::SwashCache,
    pub viewport: glyphon::Viewport,
    pub atlas: glyphon::TextAtlas,
    pub text_renderer: glyphon::TextRenderer,
    pub text_buffer: glyphon::Buffer,
}

impl GameStateOverworldDialog {
    fn init_font_system() -> glyphon::FontSystem {
        glyphon::FontSystem::new()
    }

    fn init_swash_cache() -> glyphon::SwashCache {
        glyphon::SwashCache::new()
    }

    fn init_cache(device: &wgpu::Device) -> glyphon::Cache {
        glyphon::Cache::new(device)
    }

    fn init_viewport(device: &wgpu::Device, cache: &glyphon::Cache) -> glyphon::Viewport {
        glyphon::Viewport::new(device, cache)
    }

    fn init_atlas(device: &wgpu::Device, queue: &wgpu::Queue, surface_format: wgpu::TextureFormat, cache: &glyphon::Cache) -> glyphon::TextAtlas {
        glyphon::TextAtlas::new(device, queue, cache, surface_format)
    }

    fn init_text_renderer(device: &wgpu::Device, atlas: &mut glyphon::TextAtlas) -> glyphon::TextRenderer {
        glyphon::TextRenderer::new(atlas, device, wgpu::MultisampleState::default(), None)
    }

    fn init_text_buffer(font_system: &mut glyphon::FontSystem) -> glyphon::Buffer {
        let metrics = glyphon::Metrics {
            font_size: 30_f32,
            line_height: 42_f32,
        };
        let mut text_buffer = glyphon::Buffer::new(font_system, metrics);

        let text = "Hello world!";
        let attrs = glyphon::Attrs::new().family(glyphon::Family::SansSerif);
        let shaping = glyphon::Shaping::Advanced;
        text_buffer.set_text(font_system, text, attrs, shaping);

        let prune = false;
        text_buffer.shape_until_scroll(font_system, prune);

        text_buffer
    }

    fn prepare_text_renderer(&mut self) -> Result<(), ErrorCode> {
        let device = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .device
        ;
        let queue = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .queue
        ;

        let text_areas = [glyphon::TextArea {
            buffer: &self.text_buffer,
            left: 10.,
            top: 10.,
            scale: 1.,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: 600,
                bottom: 160,
            },
            default_color: glyphon::Color::rgba(255,255,255, 255),
            custom_glyphs: &[],
        }];

        if let Err(err) = self.text_renderer.prepare(
            device, 
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            text_areas,
            &mut self.swash_cache
        ) {
            error!("Failed to prepare the text renderer: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }
}

impl Default for GameStateOverworldDialog {
    fn default() -> Self {
        let device = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .device
        ;
        let queue = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .queue
        ;
        let surface_format = get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .config
            .lock()
            .unwrap()
            .format
            .clone()
        ;

        let mut font_system = Self::init_font_system();
        let swash_cache = Self::init_swash_cache();
        let cache = Self::init_cache(device);
        let viewport = Self::init_viewport(device, &cache);
        let mut atlas = Self::init_atlas(device, queue, surface_format, &cache);
        let text_renderer = Self::init_text_renderer(device, &mut atlas);
        let text_buffer = Self::init_text_buffer(&mut font_system);

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
        }
    }
}

impl GameState for GameStateOverworldDialog {
    fn get_type(&self) -> GameStateType {
        GameStateType::OverworldDialog
    }

    fn on_update(&mut self, _keys: &HashMap<Key, KeyState>, _delta_time: &Duration) {}

    fn on_exit(&mut self) {}

    fn on_enter(&mut self) {}

    fn on_keyboard_input(&mut self, _cur_keys: &HashMap<Key, KeyState>, _old_keys: &HashMap<Key, KeyState>, _new_key: &Key, _new_key_state: &KeyState) {}

    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        self.atlas.trim();
        // Needed to get the correct image format
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

        self.prepare_text_renderer()?;

        if let Err(err) = self.text_renderer.render(&self.atlas, &self.viewport, &mut render_pass) {
            error!("Failed to render text: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        let new_resolution = glyphon::Resolution{
            width: new_width as u32,
            height: new_height as u32,
        };

        let queue = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .queue
        ;
        self.viewport.update(queue, new_resolution);
    }
}
