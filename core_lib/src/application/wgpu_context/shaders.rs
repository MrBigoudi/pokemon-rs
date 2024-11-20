pub struct Shader;

use common_lib::debug::ErrorCode;
use log::error;
use std::path::Path;

impl Shader {
    pub async fn get_shader_module(
        label: Option<&str>,
        path: &Path,
        device: &wgpu::Device,
    ) -> Result<wgpu::ShaderModule, ErrorCode> {
        let content = match common_lib::io::load_string(path).await {
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read the shader `{:?}`: {:?}", path, err);
                return Err(ErrorCode::IO);
            }
        };

        let source = wgpu::ShaderSource::Wgsl(content.into());
        Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source }))
    }
}
