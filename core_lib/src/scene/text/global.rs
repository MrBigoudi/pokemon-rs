use log::error;

use crate::utils::debug::ErrorCode;

use super::font_system::FontSystem;

use std::sync::{Mutex, OnceLock};

static GLOBAL_FONT_SYSTEM: OnceLock<Mutex<FontSystem>> = OnceLock::new();

pub fn get_global_font_system() -> Result<&'static Mutex<FontSystem>, ErrorCode> {
    match GLOBAL_FONT_SYSTEM.get() {
        Some(font_system) => Ok(font_system),
        None => {
            let new_font_system = Mutex::new(FontSystem::new()?);
            if GLOBAL_FONT_SYSTEM.set(new_font_system).is_err() {
                error!("Failed to set the global font system");
                return Err(ErrorCode::Unknown);
            };
            get_global_font_system()
        }
    }
}

pub fn resize_global_font_system(new_width: f32, new_height: f32) {
    // Update font system
    get_global_font_system()
        .unwrap()
        .lock()
        .unwrap()
        .on_resize(new_width, new_height);
}
