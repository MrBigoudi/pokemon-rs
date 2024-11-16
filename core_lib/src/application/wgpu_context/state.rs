use std::{path::PathBuf, sync::{Arc, Mutex}};

#[cfg(not(target_arch = "wasm32"))]
use location_macros::workspace_dir;

use log::{error, warn};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{application::{parameters::ApplicationParameters, utils::debug::ErrorCode}, scene::geometry::vertex::Vertex};

use super::shaders::Shader;

pub struct State {
    pub size: Mutex<PhysicalSize<u32>>,
    pub surface: wgpu::Surface<'static>,
    pub config: Mutex<wgpu::SurfaceConfiguration>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Arc<Window>,

    // TODO: Update this
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl State {
    fn init_instance() -> wgpu::Instance {
        // The instance purpose is to create Adapters and Surfaces
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        })
    }

    fn init_surface(
        instance: &wgpu::Instance,
        window: Arc<Window>,
    ) -> Result<wgpu::Surface<'static>, ErrorCode> {
        // The surface is the part of the window that we draw to
        match instance.create_surface(window) {
            Ok(surface) => Ok(surface),
            Err(err) => {
                error!(
                    "Failed to create a wgpu surface from an instance: {:?}",
                    err
                );
                Err(ErrorCode::Wgpu)
            }
        }
    }

    async fn init_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'static>,
    ) -> Result<wgpu::Adapter, ErrorCode> {
        // The adapter is a handle for the graphics card
        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // LowPower will pick an adapter that favors battery life
                // HighPerformance will pick an adapter that favors more performant GPU's
                power_preference: wgpu::PowerPreference::LowPower,
                // Tells wgpu to find an adapter that can present to the supplied surface
                compatible_surface: Some(surface),
                // Forces wgpu to pick an adapter that will work on all hardware
                // This means that the rendering backend will use a "software" system instead of hardware such as a GPU
                force_fallback_adapter: false,
            })
            .await
        {
            Some(adapter) => Ok(adapter),
            None => {
                error!("Failed to create a wgpu adapter");
                Err(ErrorCode::Wgpu)
            }
        }
    }

    async fn init_device_and_queue(
        adapter: &wgpu::Adapter,
    ) -> Result<(wgpu::Device, wgpu::Queue), ErrorCode> {
        match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features
                    // Need to disable some if building for the web
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    // Provides the adapter with a preferred memory allocation strategy
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
        {
            Ok((device, queue)) => Ok((device, queue)),
            Err(err) => {
                error!("Failed to create wgpu device and queue: {:?}", err);
                Err(ErrorCode::Wgpu)
            }
        }
    }

    fn init_size(parameters: &ApplicationParameters, window: Arc<Window>) -> PhysicalSize<u32> {
        let mut size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            warn!("The size must be greater than 0 when configuring the surface, default back to initial parameters");
            size = PhysicalSize::new(
                parameters.window_width as u32,
                parameters.window_height as u32,
            );
        }
        size
    }

    fn init_surface_config(
        surface: &wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
        size: &PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let present_mode = if surface_caps
            .present_modes
            .contains(&wgpu::PresentMode::Mailbox)
        {
            wgpu::PresentMode::Mailbox
        } else {
            wgpu::PresentMode::Fifo
        };

        let alpha_mode = if surface_caps
            .alpha_modes
            .contains(&wgpu::CompositeAlphaMode::Auto)
        {
            wgpu::CompositeAlphaMode::Auto
        } else {
            wgpu::CompositeAlphaMode::Opaque
        };

        wgpu::SurfaceConfiguration {
            // Describes how the surface textures will be used
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // Describes how the textures will be stored on the GPU
            format: surface_format,
            // Dimension in pixels of the texture
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn init_render_pipeline(device: &wgpu::Device, config: &Mutex<wgpu::SurfaceConfiguration>) -> Result<wgpu::RenderPipeline, ErrorCode> {
        // Create the shader
        #[cfg(not(target_arch = "wasm32"))]
        let mut shader_path = PathBuf::from(workspace_dir!());

        #[cfg(target_arch = "wasm32")]
        let mut shader_path = PathBuf::from("/");
        shader_path.push("shaders");
        shader_path.push("default");
        shader_path.set_extension("wgsl");

        let shader = match Shader::get_shader_module("DefaultShader", &shader_path, device){
            Ok(shader) => shader,
            Err(err) => {
                error!("Failed to create the render pipeline's shader module: {:?}", err);
                return Err(ErrorCode::Wgpu);
            }
        };

        // Create the pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("DefaultPipelineLayout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create the pipeline
        let format = config.lock().unwrap().format;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("DefaultPipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout()], // Type of vertices passed to the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
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

    fn init_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("TriangleVertexBuffer"),
            contents: bytemuck::cast_slice(crate::scene::geometry::vertex::TRIANGLE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        vertex_buffer
    }

    fn init_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("TriangleIndexBuffer"),
            contents: bytemuck::cast_slice(crate::scene::geometry::vertex::TRIANGLE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        vertex_buffer
    }


    pub async fn new(
        parameters: &ApplicationParameters,
        window: Arc<Window>,
    ) -> Result<State, ErrorCode> {
        let instance = Self::init_instance();
        let surface = Self::init_surface(&instance, Arc::clone(&window))?;
        let adapter = Self::init_adapter(&instance, &surface).await?;
        let (device, queue) = Self::init_device_and_queue(&adapter).await?;
        let size = Self::init_size(parameters, Arc::clone(&window));
        let config = Mutex::new(Self::init_surface_config(&surface, &adapter, &size));
        let size = Mutex::new(size);
        
        let vertex_buffer = Self::init_vertex_buffer(&device);
        let index_buffer = Self::init_index_buffer(&device);
        let render_pipeline = Self::init_render_pipeline(&device, &config)?;

        Ok(Self {
            size,
            surface,
            config,
            device,
            queue,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
        })
    }
}
