use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use location_macros::workspace_dir;

use crate::{application::{utils::debug::ErrorCode, wgpu_context::pipelines::{graphics::{GraphicsPipeline, GraphicsPipelineBase}, PipelineResources}}, scene::{geometry::vertex::Vertex, rendering::texture}};

pub struct DefaultGraphicsPipelineResources {
    pub diffuse_texture: texture::Texture,
}

impl PipelineResources for DefaultGraphicsPipelineResources{}

#[non_exhaustive]
pub struct DefaultGraphicsPipeline {
    pub base: GraphicsPipelineBase,
    pub resources: DefaultGraphicsPipelineResources,
}

impl DefaultGraphicsPipeline {
    pub fn new() -> Result<Self, ErrorCode> {
        // Create the shader
        #[cfg(not(target_arch = "wasm32"))]
        let mut shader_path = PathBuf::from(workspace_dir!());

        #[cfg(target_arch = "wasm32")]
        let mut shader_path = PathBuf::from("/");
        shader_path.push("shaders");
        shader_path.push("default");
        shader_path.set_extension("wgsl");

        let (resources, base) = Self::from_single_shader_path(&shader_path, None, "vs_main", "fs_main")?;
        Ok(Self{
            base,
            resources,
        })
    }
}

impl GraphicsPipeline for DefaultGraphicsPipeline {
    type Resources = DefaultGraphicsPipelineResources;
    type Pipeline = DefaultGraphicsPipeline;

    fn get_resources(&self) -> &Self::Resources {
        &self.resources
    }

    fn get_base(&self) -> &GraphicsPipelineBase {
        &self.base
    }

    fn set_base(&mut self, base: GraphicsPipelineBase) {
        self.base = base;
    }

    fn init_resources() -> Result<Self::Resources, ErrorCode> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut texture_path = PathBuf::from(workspace_dir!());

        #[cfg(target_arch = "wasm32")]
        let mut texture_path = PathBuf::from("/");
        texture_path.push("assets");
        texture_path.push("sprites");
        texture_path.push("pokemons");
        texture_path.push("bulbasaur");
        texture_path.push("front");
        texture_path.set_extension("png");

        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let diffuse_texture = texture::Texture::from_path(&texture_path, &global_wgpu_state.device, &global_wgpu_state.queue, None)?;
        Ok(DefaultGraphicsPipelineResources{
            diffuse_texture,
        })
    }

    fn init_bind_groups(resources: &Self::Resources, bind_groups_layouts: &Vec<wgpu::BindGroupLayout>) -> Result<Vec<wgpu::BindGroup>, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;

        let bind_group_0 = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_groups_layouts[0],
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&resources.diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&resources.diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        Ok(vec![bind_group_0])
    }

    fn init_bind_groups_layouts() -> Result<Vec<wgpu::BindGroupLayout>, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;
        let bind_group_0_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Sampled texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("bind_group_0_layout"),
        });

        Ok(vec![bind_group_0_layout])
    }

    fn init_render_pipeline_from_multiple_modules(
        vertex_shader_module: wgpu::ShaderModule, 
        fragment_shader_module: wgpu::ShaderModule,
        bind_groups_layouts: &Vec<&wgpu::BindGroupLayout>,
    ) -> Result<wgpu::RenderPipeline, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;

        // Create the pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("DefaultPipelineLayout"),
            bind_group_layouts: bind_groups_layouts,
            push_constant_ranges: &[],
        });

        // Create the pipeline
        let config = &global_wgpu_state.config;
        let format = config.lock().unwrap().format;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("DefaultPipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: None,
                buffers: &[Vertex::layout()], // Type of vertices passed to the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &fragment_shader_module,
                entry_point: None,
                targets: &[Some(wgpu::ColorTargetState{
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL, // Write to all channels
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // Number of samples
                mask: !0, // Active samples (here all)
                alpha_to_coverage_enabled: false,
            },
            multiview: None, // Number of array layers, (here not rendering to array textures)
            cache: None, // Cache shader compilation data (Only useful for Android)
        });

        Ok(pipeline)
    }

    fn init_render_pipeline_from_single_module(
        shader_module: wgpu::ShaderModule, 
        vertex_entry_point: &str,
        fragment_entry_point: &str,
        bind_groups_layouts: &Vec<&wgpu::BindGroupLayout>,
    ) -> Result<wgpu::RenderPipeline, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;

        // Create the pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("DefaultPipelineLayout"),
            bind_group_layouts: bind_groups_layouts,
            push_constant_ranges: &[],
        });

        // Create the pipeline
        let config = &global_wgpu_state.config;
        let format = config.lock().unwrap().format;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("DefaultPipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(vertex_entry_point),
                buffers: &[Vertex::layout()], // Type of vertices passed to the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader_module,
                entry_point: Some(fragment_entry_point),
                targets: &[Some(wgpu::ColorTargetState{
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL, // Write to all channels
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // Number of samples
                mask: !0, // Active samples (here all)
                alpha_to_coverage_enabled: false,
            },
            multiview: None, // Number of array layers, (here not rendering to array textures)
            cache: None, // Cache shader compilation data (Only useful for Android)
        });

        Ok(pipeline)
    }
}