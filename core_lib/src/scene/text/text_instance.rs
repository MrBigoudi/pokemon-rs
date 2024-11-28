use std::{ops::DerefMut, sync::Arc};

use log::error;
use wgpu::RenderPass;

use super::{font::FontFamily, global::get_global_font_system};
use crate::{
    utils::debug::ErrorCode,
    wgpu_context::{global::get_global_wgpu_state, state::State},
};

pub struct TextParameters {
    // Appearance paramters
    pub content: String,
    pub font_size: f32,
    pub line_height: f32,
    pub font_family: FontFamily,

    pub font_color: [u8; 4],
    // Text bounds positions
    pub bounds_left: f32,
    pub bounds_top: f32,
    // Text bounds dimensions
    pub bounds_width: u16,
    pub bounds_height: u16,
}

pub struct TextInstance {
    parameters: TextParameters,
    buffer: glyphon::Buffer,
}

impl TextInstance {
    pub fn new(parameters: TextParameters) -> Result<Self, ErrorCode> {
        let buffer = Self::init_buffer(&parameters)?;
        Ok(Self { parameters, buffer })
    }

    pub fn update(&mut self, new_parameters: TextParameters) -> Result<(), ErrorCode> {
        *self = Self::new(new_parameters)?;
        Ok(())
    }

    /// Fails if the global wgpu state was not initialized
    fn get_global_wgpu_state() -> Result<Arc<State>, ErrorCode> {
        match get_global_wgpu_state() {
            Ok(state) => Ok(state),
            Err(err) => {
                error!(
                    "Failed to get the global wgpu state in a text instance: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    /// Fails if the global font system was not initialized
    fn init_buffer(paramters: &TextParameters) -> Result<glyphon::Buffer, ErrorCode> {
        let metrics = glyphon::Metrics {
            font_size: paramters.font_size,
            line_height: paramters.line_height,
        };
        let mut font_system = match get_global_font_system() {
            Ok(system) => match system.lock() {
                Ok(locked) => locked,
                Err(err) => {
                    error!("Failed to lock the global font system when initializing a text instance buffer: {:?}", err);
                    return Err(ErrorCode::SyncError);
                }
            },
            Err(err) => {
                error!("Failed to get the global font system when initializing a text instance buffer: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };
        let font_system = font_system.handle.get_mut();

        let mut text_buffer = glyphon::Buffer::new(font_system, metrics);
        let text = &paramters.content;
        let attrs = glyphon::Attrs::new().family(paramters.font_family.to_glyphon());
        let shaping = glyphon::Shaping::Advanced;
        text_buffer.set_text(font_system, text, attrs, shaping);
        let prune = false;
        text_buffer.shape_until_scroll(font_system, prune);

        Ok(text_buffer)
    }

    /// Fails if the global wgpu state was not initialized
    /// Fails if the global font system was not initialized
    pub fn render(&self, mut render_pass: RenderPass<'_>) -> Result<(), ErrorCode> {
        let text_areas = [glyphon::TextArea {
            buffer: &self.buffer,
            left: self.parameters.bounds_left,
            top: self.parameters.bounds_top,
            scale: 1.,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: self.parameters.bounds_width as i32,
                bottom: self.parameters.bounds_height as i32,
            },
            default_color: glyphon::Color::rgba(
                self.parameters.font_color[0],
                self.parameters.font_color[1],
                self.parameters.font_color[2],
                self.parameters.font_color[3],
            ),
            custom_glyphs: &[],
        }];

        let gpu_state = Self::get_global_wgpu_state()?;
        let device = &gpu_state.device;
        let queue = &gpu_state.queue;

        let font_system = match get_global_font_system() {
            Ok(system) => match system.lock() {
                Ok(locked) => locked,
                Err(err) => {
                    error!("Failed to lock the global font system when initializing a text instance buffer: {:?}", err);
                    return Err(ErrorCode::SyncError);
                }
            },
            Err(err) => {
                error!("Failed to get the global font system when initializing a text instance buffer: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        let mut text_renderer = font_system.text_renderer.borrow_mut();
        let mut handle = font_system.handle.borrow_mut();
        let mut swash_cache = font_system.swash_cache.borrow_mut();
        let mut atlas = font_system.atlas.borrow_mut();
        let viewport = font_system.viewport.borrow();

        if let Err(err) = text_renderer.prepare(
            device,
            queue,
            handle.deref_mut(),
            atlas.deref_mut(),
            &viewport,
            text_areas,
            swash_cache.deref_mut(),
        ) {
            error!("Failed to prepare the text renderer: {:?}", err);
            return Err(ErrorCode::Unknown);
        };

        if let Err(err) = text_renderer.render(&atlas, &viewport, &mut render_pass) {
            error!("Failed to render text: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        atlas.trim();

        Ok(())
    }
}
