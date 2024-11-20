use std::path::Path;

use common_lib::debug::ErrorCode;
use image::GenericImageView;
use log::error;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub async fn from_path(
        path: &Path,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Result<Self, ErrorCode> {
        let content = match common_lib::io::load_bytes(path).await {
            Ok(bytes) => bytes,
            Err(err) => {
                error!(
                    "Failed to read the texture from path `{:?}`: {:?}",
                    path, err
                );
                return Err(ErrorCode::IO);
            }
        };

        match Self::from_bytes(&content, device, queue, label) {
            Ok(texture) => Ok(texture),
            Err(err) => {
                error!(
                    "Failed to create the texture from path `{:?}`: {:?}",
                    path, err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    pub fn from_bytes(
        bytes: &[u8],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Result<Self, ErrorCode> {
        let diffuse_image = match image::load_from_memory(bytes) {
            Ok(image) => image,
            Err(err) => {
                error!("Failed to load the texture from bytes: {:?}", err);
                return Err(ErrorCode::IO);
            }
        };

        match Self::from_image(&diffuse_image, device, queue, label) {
            Ok(texture) => Ok(texture),
            Err(err) => {
                error!("Failed to create the texture from bytes: {:?}", err);
                Err(ErrorCode::Unknown)
            }
        }
    }

    pub fn from_image(
        image: &image::DynamicImage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Result<Self, ErrorCode> {
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        // All textures are stored as 3D
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1, // For 2D
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // Bindings = shader usable, Copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The pixel data
            &rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest, // For pixelated look
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
