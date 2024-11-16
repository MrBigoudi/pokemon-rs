pub struct Shader;

use std::path::Path;
use crate::application::utils::debug::ErrorCode;
use log::error;

impl Shader {
    #[cfg(not(target_arch = "wasm32"))]
    fn get_source(path: &Path) -> Result<wgpu::ShaderSource, ErrorCode> {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read the shader `{:?}`: {:?}", path, err);
                return Err(ErrorCode::IO);
            }
        };
        Ok(wgpu::ShaderSource::Wgsl(content.into()))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_shader_module(
        name: &str,
        path: &Path,
        device: &wgpu::Device,
    ) -> Result<wgpu::ShaderModule, ErrorCode> {
        let source = Self::get_source(path)?;
        let label = Some(name);
        Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source,
        }))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_shader_module(
        name: &str,
        url: &Path,
        device: &wgpu::Device,
    ) -> Result<wgpu::ShaderModule, ErrorCode> {
        let url: &str = url.as_os_str().to_str().unwrap();
        let shader_code = match pollster::block_on(Self::fetch_shader(url)) {
            Ok(code) => code,
            Err(err) => {
                error!(
                    "Failed to fetch the shader `{}' at `{}': {:?}",
                    name, url, err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let label = Some(name);
        Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        }))
    }

    #[cfg(target_arch = "wasm32")]
    async fn fetch_shader(url: &str) -> Result<String, ErrorCode> {
        let response = reqwest::get(url).await.map_err(|err| {
            error!("Failed to fetch shader `{}`: {:?}", url, err);
            ErrorCode::Network
        })?;

        if response.status().is_success() {
            let text = response.text().await.map_err(|err| {
                error!("Failed to read shader response text from `{}`: {:?}", url, err);
                ErrorCode::IO
            })?;
            Ok(text)
        } else {
            error!("Failed to fetch shader `{}`: HTTP {}", url, response.status());
            Err(ErrorCode::Network)
        }
    }
}