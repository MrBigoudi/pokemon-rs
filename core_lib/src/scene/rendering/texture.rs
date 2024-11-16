use std::path::Path;

use image::GenericImageView;
use log::error;

use crate::application::utils::debug::ErrorCode;

pub struct Texture{
    pub image_rgba: image::RgbaImage,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn new(path: &Path) -> Result<Self, ErrorCode> {
        let diffuse_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => {
                error!("Failed to read the texture `{:?}`: {:?}", path, err);
                return Err(ErrorCode::IO);
            }
        };

        let diffuse_image = match image::load_from_memory(&diffuse_bytes){
            Ok(image) => image,
            Err(err) => {
                error!("Failed to load the texture from `{:?}': {:?}", path, err);
                return Err(ErrorCode::IO);
            }
        };
        
        let image_rgba = diffuse_image.to_rgba8();
        let dimensions = diffuse_image.dimensions();

        Ok(Self {
            image_rgba,
            dimensions,
        })
    }
}