use std::cell::RefCell;
use std::sync::Arc;

use log::error;

use crate::{
    utils::debug::ErrorCode,
    wgpu_context::{global::get_global_wgpu_state, state::State},
};

#[non_exhaustive]
pub struct FontSystem {
    pub handle: RefCell<glyphon::FontSystem>,
    pub swash_cache: RefCell<glyphon::SwashCache>,
    pub atlas: RefCell<glyphon::TextAtlas>,
    pub text_renderer: RefCell<glyphon::TextRenderer>,
    pub viewport: RefCell<glyphon::Viewport>,
}

impl FontSystem {
    /// Fails if the global wgpu state was not initialized
    fn get_global_wgpu_state() -> Result<Arc<State>, ErrorCode> {
        match get_global_wgpu_state() {
            Ok(state) => Ok(state),
            Err(err) => {
                error!(
                    "Failed to get the global wgpu state to initialize the font system: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    pub fn on_resize(&mut self, new_width: f32, new_height: f32) {
        let new_resolution = glyphon::Resolution {
            width: new_width as u32,
            height: new_height as u32,
        };

        let queue = &get_global_wgpu_state()
            .expect("Failed to fetch the global gpu state")
            .queue;
        self.viewport.borrow_mut().update(queue, new_resolution);
    }

    /// Fails if the global wgpu state has not been initialized
    pub fn new() -> Result<Self, ErrorCode> {
        let gpu_state = Self::get_global_wgpu_state()?;
        let device = &gpu_state.device;
        let queue = &gpu_state.queue;
        let surface_format = gpu_state.config.lock().unwrap().format;

        let handle = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(device);
        let viewport = glyphon::Viewport::new(device, &cache);
        let mut atlas = glyphon::TextAtlas::new(device, queue, &cache, surface_format);
        let text_renderer =
            glyphon::TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        Ok(Self {
            handle: RefCell::new(handle),
            swash_cache: RefCell::new(swash_cache),
            atlas: RefCell::new(atlas),
            text_renderer: RefCell::new(text_renderer),
            viewport: RefCell::new(viewport),
        })
    }
}
