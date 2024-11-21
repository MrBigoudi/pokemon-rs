use std::{path::Path, sync::Arc};

use common_lib::debug::ErrorCode;
use log::error;

use crate::{
    application::wgpu_context::{shaders::Shader, state::State},
    global,
    scene::Scene,
};

use super::PipelineResources;

#[derive(Debug)]
pub struct GraphicsPipelineBase {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub bind_groups_layouts: Vec<wgpu::BindGroupLayout>,
}

pub trait GraphicsPipeline {
    type Resources: PipelineResources;
    type Pipeline: GraphicsPipeline;

    fn get_resources(&self) -> &Self::Resources;
    fn get_base(&self) -> &GraphicsPipelineBase;
    fn set_base(&mut self, base: GraphicsPipelineBase);

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

    fn get_global_scene() -> Result<Arc<Scene>, ErrorCode> {
        match global::get_global_scene() {
            Ok(scene) => Ok(scene),
            Err(err) => {
                error!(
                    "Failed to get the global scene when creating a graphics pipeline: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    #[allow(async_fn_in_trait)]
    async fn from_multiple_shader_paths(
        vertex_shader_path: &Path,
        fragment_shader_path: &Path,
        vertex_label: Option<&str>,
        fragment_label: Option<&str>,
    ) -> Result<(Self::Resources, GraphicsPipelineBase), ErrorCode> {
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

        Self::from_multiple_shader_modules(vertex_module, fragment_module).await
    }

    #[allow(async_fn_in_trait)]
    async fn from_single_shader_path(
        shader_path: &Path,
        shader_label: Option<&str>,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
    ) -> Result<(Self::Resources, GraphicsPipelineBase), ErrorCode> {
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

        Self::from_single_shader_module(shader_module, vertex_entry_point, fragment_entry_point)
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn from_multiple_shader_modules(
        vertex_shader_module: wgpu::ShaderModule,
        fragment_shader_module: wgpu::ShaderModule,
    ) -> Result<(Self::Resources, GraphicsPipelineBase), ErrorCode> {
        let resources = match Self::init_resources().await {
            Ok(resources) => resources,
            Err(err) => {
                error!(
                    "Failed to init the resources when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups_layouts = match Self::init_bind_groups_layouts() {
            Ok(layouts) => layouts,
            Err(err) => {
                error!("Failed to init the bind groups layouts when creating a graphics pipeline: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups = match Self::init_bind_groups(&resources, &bind_groups_layouts) {
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
            vertex_shader_module,
            fragment_shader_module,
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

        Ok((resources, base))
    }

    #[allow(async_fn_in_trait)]
    async fn from_single_shader_module(
        shader_module: wgpu::ShaderModule,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
    ) -> Result<(Self::Resources, GraphicsPipelineBase), ErrorCode> {
        let resources = match Self::init_resources().await {
            Ok(resources) => resources,
            Err(err) => {
                error!(
                    "Failed to init the resources when creating a graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups_layouts = match Self::init_bind_groups_layouts() {
            Ok(layouts) => layouts,
            Err(err) => {
                error!("Failed to init the bind groups layouts when creating a graphics pipeline: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        let bind_groups = match Self::init_bind_groups(&resources, &bind_groups_layouts) {
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
            shader_module,
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

        Ok((resources, base))
    }

    #[allow(async_fn_in_trait)]
    async fn init_resources() -> Result<Self::Resources, ErrorCode>;
    fn init_bind_groups_layouts() -> Result<Vec<wgpu::BindGroupLayout>, ErrorCode>;
    fn init_bind_groups(
        resources: &Self::Resources,
        bind_groups_layouts: &[wgpu::BindGroupLayout],
    ) -> Result<Vec<wgpu::BindGroup>, ErrorCode>;
    fn init_render_pipeline_from_multiple_modules(
        vertex_shader_module: wgpu::ShaderModule,
        fragment_shader_module: wgpu::ShaderModule,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
    ) -> Result<wgpu::RenderPipeline, ErrorCode>;
    fn init_render_pipeline_from_single_module(
        shader_module: wgpu::ShaderModule,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
    ) -> Result<wgpu::RenderPipeline, ErrorCode>;
}
