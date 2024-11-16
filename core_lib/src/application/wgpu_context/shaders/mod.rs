use std::{fs::read_to_string, path::Path};

use log::error;

use crate::application::utils::debug::ErrorCode;

pub struct Shader;

impl Shader {
    fn get_source(path: &Path) -> Result<wgpu::ShaderSource, ErrorCode> {
        let content = match read_to_string(path){
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read the shader `{:?}': {:?}", path, err);
                return Err(ErrorCode::I0);
            }
        };
        Ok(wgpu::ShaderSource::Wgsl(content.into()))
    }

    pub fn get_shader_module(name: &str, path: &Path, device: &wgpu::Device) -> Result<wgpu::ShaderModule, ErrorCode>{
        let source = Self::get_source(path)?;
        let label = Some(name);
        Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label,
            source,
        }))
    }
}