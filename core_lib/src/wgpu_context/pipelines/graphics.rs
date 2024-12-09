use std::{path::Path, sync::Arc};

use log::error;

use crate::{
    utils::debug::ErrorCode,
    wgpu_context::{global, shaders::Shader, state::State},
};

use super::PipelineResources;

pub struct GraphicsPipelineBase {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub bind_groups_layouts: Vec<wgpu::BindGroupLayout>,
}

pub trait GraphicsPipeline {
    type Resources: PipelineResources;
    type Pipeline: GraphicsPipeline;

    fn get_global_wgpu_state() -> Result<Arc<State>, ErrorCode> {
        match global::get_global_wgpu_state() {
            Ok(state) => Ok(state),
            Err(err) => {
                error!(
                    "Failed to get the global wgpu state when creating a graphics pipeline: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    #[allow(async_fn_in_trait)]
    async fn from_multiple_shader_paths(
        resources: &Self::Resources,
        vertex_shader_path: &Path,
        fragment_shader_path: &Path,
        vertex_label: Option<&str>,
        fragment_label: Option<&str>,
    ) -> Result<GraphicsPipelineBase, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;

        let vertex_module = match Shader::get_shader_module(
            vertex_label,
            vertex_shader_path,
            &global_wgpu_state.device,
        )
        .await
        {
            Ok(shader) => shader,
            Err(err) => {
                error!(
                    "Failed to create the render pipeline's vertex shader module `{:?}': {:?}",
                    vertex_shader_path, err
                );
                return Err(ErrorCode::Wgpu);
            }
        };

        let fragment_module = match Shader::get_shader_module(
            fragment_label,
            fragment_shader_path,
            &global_wgpu_state.device,
        )
        .await
        {
            Ok(shader) => shader,
            Err(err) => {
                error!(
                    "Failed to create the render pipeline's fragment shader module `{:?}': {:?}",
                    fragment_shader_path, err
                );
                return Err(ErrorCode::Wgpu);
            }
        };

        Self::from_multiple_shader_modules(resources, vertex_module, fragment_module)
    }

    #[allow(async_fn_in_trait)]
    async fn from_single_shader_path(
        resources: &Self::Resources,
        shader_path: &Path,
        shader_label: Option<&str>,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
    ) -> Result<GraphicsPipelineBase, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;

        let shader_module =
            match Shader::get_shader_module(shader_label, shader_path, &global_wgpu_state.device)
                .await
            {
                Ok(shader) => shader,
                Err(err) => {
                    error!(
                        "Failed to create the render pipeline's shader module `{:?}': {:?}",
                        shader_path, err
                    );
                    return Err(ErrorCode::Wgpu);
                }
            };

        Self::from_single_shader_module(
            resources,
            shader_module,
            vertex_entry_point,
            fragment_entry_point,
        )
    }

    fn from_multiple_shader_modules(
        resources: &Self::Resources,
        vertex_shader_module: wgpu::ShaderModule,
        fragment_shader_module: wgpu::ShaderModule,
    ) -> Result<GraphicsPipelineBase, ErrorCode> {
        let bind_groups_layouts = match Self::init_bind_groups_layouts() {
            Ok(layouts) => layouts,
            Err(err) => {
                error!("Failed to init the bind groups layouts when creating a graphics pipeline: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups = match Self::init_bind_groups(resources, &bind_groups_layouts) {
            Ok(groups) => groups,
            Err(err) => {
                error!(
                    "Failed to init the bind groups when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let mut group_layouts = Vec::new();
        for group_layout in &bind_groups_layouts {
            group_layouts.push(group_layout);
        }
        let render_pipeline = match Self::init_render_pipeline_from_multiple_modules(
            &vertex_shader_module,
            &fragment_shader_module,
            None,
            None,
            &group_layouts,
        ) {
            Ok(groups) => groups,
            Err(err) => {
                error!(
                    "Failed to init the render pipeline when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let base = GraphicsPipelineBase {
            render_pipeline,
            bind_groups,
            bind_groups_layouts,
        };

        Ok(base)
    }

    fn from_single_shader_module(
        resources: &Self::Resources,
        shader_module: wgpu::ShaderModule,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
    ) -> Result<GraphicsPipelineBase, ErrorCode> {
        let bind_groups_layouts = match Self::init_bind_groups_layouts() {
            Ok(layouts) => layouts,
            Err(err) => {
                error!("Failed to init the bind groups layouts when creating a graphics pipeline: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups = match Self::init_bind_groups(resources, &bind_groups_layouts) {
            Ok(groups) => groups,
            Err(err) => {
                error!(
                    "Failed to init the bind groups when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let mut group_layouts = Vec::new();
        for group_layout in &bind_groups_layouts {
            group_layouts.push(group_layout);
        }
        let render_pipeline = match Self::init_render_pipeline_from_single_module(
            &shader_module,
            vertex_entry_point,
            fragment_entry_point,
            &group_layouts,
        ) {
            Ok(groups) => groups,
            Err(err) => {
                error!(
                    "Failed to init the render pipeline when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let base = GraphicsPipelineBase {
            render_pipeline,
            bind_groups,
            bind_groups_layouts,
        };

        Ok(base)
    }

    fn init_render_pipeline_from_single_module(
        shader_module: &wgpu::ShaderModule,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
    ) -> Result<wgpu::RenderPipeline, ErrorCode> {
        Self::init_render_pipeline_from_multiple_modules(
            shader_module,
            shader_module,
            Some(vertex_entry_point),
            Some(fragment_entry_point),
            bind_groups_layouts,
        )
    }

    // Functions to reimplement
    fn get_base(&self) -> &GraphicsPipelineBase;
    fn set_base(&mut self, base: GraphicsPipelineBase);

    fn init_bind_groups_layouts() -> Result<Vec<wgpu::BindGroupLayout>, ErrorCode>;

    fn init_bind_groups(
        resources: &Self::Resources,
        bind_groups_layouts: &[wgpu::BindGroupLayout],
    ) -> Result<Vec<wgpu::BindGroup>, ErrorCode>;

    fn init_render_pipeline_from_multiple_modules(
        vertex_shader_module: &wgpu::ShaderModule,
        fragment_shader_module: &wgpu::ShaderModule,
        vertex_entry_point: Option<&str>,
        fragment_entry_point: Option<&str>,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
    ) -> Result<wgpu::RenderPipeline, ErrorCode>;
}
