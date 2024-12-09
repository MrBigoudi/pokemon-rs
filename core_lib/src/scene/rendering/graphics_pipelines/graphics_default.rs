use std::path::PathBuf;

use crate::{
    scene::{geometry::vertex::Vertex, rendering::texture::Texture},
    utils::debug::ErrorCode,
    wgpu_context::pipelines::{
        graphics::{GraphicsPipeline, GraphicsPipelineBase},
        PipelineResources,
    },
};

pub struct DefaultGraphicsPipelineResources {
    pub diffuse_texture: Texture,
    pub camera_buffer: wgpu::Buffer,
}

impl PipelineResources for DefaultGraphicsPipelineResources {}

#[non_exhaustive]
pub struct DefaultGraphicsPipeline {
    pub base: GraphicsPipelineBase,
}

impl DefaultGraphicsPipeline {
    pub async fn new(resources: &DefaultGraphicsPipelineResources) -> Result<Self, ErrorCode> {
        // Create the shader
        let mut shader_path = PathBuf::from("");
        shader_path.push("shaders");
        shader_path.push("default");
        shader_path.set_extension("wgsl");

        let base =
            Self::from_single_shader_path(resources, &shader_path, None, "vs_main", "fs_main")
                .await?;

        Ok(Self { base })
    }

    fn init_bind_group_0_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Diffuse texture Sampled
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
                // Diffuse texture Sampler
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
        })
    }

    fn init_bind_group_1_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Camera UBO
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("bind_group_1_layout"),
        })
    }

    fn init_bind_group_0(
        device: &wgpu::Device,
        resources: &DefaultGraphicsPipelineResources,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resources.diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&resources.diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_texture_bind_group"),
        })
    }

    fn init_bind_group_1(
        device: &wgpu::Device,
        resources: &DefaultGraphicsPipelineResources,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: resources.camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_ubo_bind_group"),
        })
    }
}

impl GraphicsPipeline for DefaultGraphicsPipeline {
    type Resources = DefaultGraphicsPipelineResources;
    type Pipeline = DefaultGraphicsPipeline;

    fn get_base(&self) -> &GraphicsPipelineBase {
        &self.base
    }

    fn set_base(&mut self, base: GraphicsPipelineBase) {
        self.base = base;
    }

    fn init_bind_groups_layouts() -> Result<Vec<wgpu::BindGroupLayout>, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;

        let bind_group_0_layout = Self::init_bind_group_0_layout(device);
        let bind_group_1_layout = Self::init_bind_group_1_layout(device);
        Ok(vec![bind_group_0_layout, bind_group_1_layout])
    }

    fn init_bind_groups(
        resources: &Self::Resources,
        bind_groups_layouts: &[wgpu::BindGroupLayout],
    ) -> Result<Vec<wgpu::BindGroup>, ErrorCode> {
        let global_wgpu_state = Self::get_global_wgpu_state()?;
        let device = &global_wgpu_state.device;

        let bind_group_0 = Self::init_bind_group_0(device, resources, &bind_groups_layouts[0]);
        let bind_group_1 = Self::init_bind_group_1(device, resources, &bind_groups_layouts[1]);

        Ok(vec![bind_group_0, bind_group_1])
    }

    fn init_render_pipeline_from_multiple_modules(
        vertex_shader_module: &wgpu::ShaderModule,
        fragment_shader_module: &wgpu::ShaderModule,
        vertex_entry_point: Option<&str>,
        fragment_entry_point: Option<&str>,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("DefaultPipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader_module,
                entry_point: vertex_entry_point,
                buffers: &[Vertex::layout()], // Type of vertices passed to the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader_module,
                entry_point: fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL, // Write to all channels
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            cache: None,     // Cache shader compilation data (Only useful for Android)
        });

        Ok(pipeline)
    }
}
