use std::collections::HashMap;

use core_lib::{
    scene::{
        animation::movement::MovementDirection, camera::{Camera, ProjectionType}, geometry::vertex::Vertex, rendering::{
            frame::FrameData,
            graphics_pipelines::graphics_default::{
                DefaultGraphicsPipeline, DefaultGraphicsPipelineResources,
            },
        }, Scene
    },
    utils::{debug::ErrorCode, time::Duration},
    wgpu_context::global::get_global_wgpu_state,
    window::key_map::{Key, KeyState},
    DeviceExt,
};

use crate::{character::player::Player, states::state::{GameState, GameStateType}};

pub struct GameStateOverworld {
    // Graphics pipeline and associated resources
    pub graphics_pipeline: DefaultGraphicsPipeline,
    pub graphics_pipeline_resources: DefaultGraphicsPipelineResources,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,

    pub scene: Scene,
    pub delta_time: Duration,

    pub player: Player,
}

impl GameStateOverworld {
    fn init_player() -> Player {
        Player::new()
    }

    fn init_vertex_buffer(device: &wgpu::Device, player: &Player) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TriangleVertexBuffer"),
            contents: bytemuck::cast_slice(&player.get_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &wgpu::Device) {
        let new_vertex_buffer = Self::init_vertex_buffer(device, &self.player);
        self.vertex_buffer = new_vertex_buffer;
    }

    fn init_index_buffer(device: &wgpu::Device, player: &Player) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TriangleIndexBuffer"),
            contents: bytemuck::cast_slice(&player.get_indices()),
            usage: wgpu::BufferUsages::INDEX,
        });
        vertex_buffer
    }

    fn init_scene(viewport_width: f32, viewport_height: f32) -> Scene {
        let camera = Camera::new(viewport_width, viewport_height);
        Scene { camera }
    }

    fn update_camera_buffer(&self) {
        let queue = &get_global_wgpu_state().unwrap().queue;

        let buffer_to_update = &self.graphics_pipeline_resources.camera_buffer;
        let offset = 0;

        let new_data = [self.scene.camera.to_camera_gpu(ProjectionType::Perspective)];
        let new_data = bytemuck::cast_slice(&new_data);

        queue.write_buffer(buffer_to_update, offset, new_data);
    }

    fn init_graphics_pipeline_resources(
        scene: &Scene,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> DefaultGraphicsPipelineResources {
        // Init the camera buffer
        let camera_gpu = scene.camera.to_camera_gpu(ProjectionType::Perspective);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[camera_gpu]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Init the diffuse texture
        let diffuse_texture = Player::get_texture(device, queue);

        DefaultGraphicsPipelineResources {
            camera_buffer,
            diffuse_texture,
        }
    }

    fn init_graphics_pipeline(
        resources: &DefaultGraphicsPipelineResources,
    ) -> DefaultGraphicsPipeline {
        pollster::block_on(DefaultGraphicsPipeline::new(resources)).unwrap()
    }
}

impl Default for GameStateOverworld {
    fn default() -> Self {
        let size = &get_global_wgpu_state().unwrap().size;
        let device = &get_global_wgpu_state().unwrap().device;
        let queue = &get_global_wgpu_state().unwrap().queue;

        let viewport_width = size.lock().unwrap().width as f32;
        let viewport_height = size.lock().unwrap().height as f32;
        let scene = Self::init_scene(viewport_width, viewport_height);
        let player = Self::init_player();

        let graphics_pipeline_resources =
            Self::init_graphics_pipeline_resources(&scene, device, queue);
        let graphics_pipeline = Self::init_graphics_pipeline(&graphics_pipeline_resources);

        let vertex_buffer = Self::init_vertex_buffer(device, &player);
        let index_buffer = Self::init_index_buffer(device, &player);


        Self {
            graphics_pipeline,
            graphics_pipeline_resources,
            vertex_buffer,
            index_buffer,
            scene,
            delta_time: Default::default(),
            player,
        }
    }
}

impl GameState for GameStateOverworld {
    fn get_type(&self) -> GameStateType {
        GameStateType::Overworld
    }

    fn on_update(&mut self, keys: &HashMap<Key, KeyState>, delta_time: &Duration) {
        self.delta_time = *delta_time;
        let device = &get_global_wgpu_state().unwrap().device;

        // Update camera
        if let Some(KeyState::Pressed) = keys.get(&Key::S) {
            self.scene
                .camera
                .on_move(MovementDirection::Forward, self.delta_time);
            // Update player state
            self.player.on_move(MovementDirection::Backward);
            self.update_vertex_buffer(device);
        }
        if let Some(KeyState::Pressed) = keys.get(&Key::D) {
            self.scene
                .camera
                .on_move(MovementDirection::Left, self.delta_time);
            // Update player state
            self.player.on_move(MovementDirection::Right);
            self.update_vertex_buffer(device);
        }
        if let Some(KeyState::Pressed) = keys.get(&Key::W) {
            self.scene
                .camera
                .on_move(MovementDirection::Backward, self.delta_time);
            // Update player state
            self.player.on_move(MovementDirection::Forward);
            self.update_vertex_buffer(device);
        }
        if let Some(KeyState::Pressed) = keys.get(&Key::A) {
            self.scene
                .camera
                .on_move(MovementDirection::Right, self.delta_time);
            // Update player state
            self.player.on_move(MovementDirection::Left);
            self.update_vertex_buffer(device);
        }
        self.update_camera_buffer();
    }

    fn on_exit(&mut self) {}

    fn on_enter(&mut self) {}

    fn on_keyboard_input(
        &mut self,
        _cur_keys: &HashMap<Key, KeyState>,
        _old_keys: &HashMap<Key, KeyState>,
        _new_key: &Key,
        _new_key_state: &KeyState,
    ) {
    }

    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        let output = &frame_data.frame_buffer;
        // Create a render pass
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder = &mut frame_data.command_buffer;
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let render_pipeline = &self.graphics_pipeline.base.render_pipeline;
        render_pass.set_pipeline(render_pipeline);

        // Diffuse texture bind group
        let bind_group_0 = &self.graphics_pipeline.base.bind_groups[0];
        render_pass.set_bind_group(0, bind_group_0, &[]);

        // Camera UBO bind group
        let bind_group_1 = &self.graphics_pipeline.base.bind_groups[1];
        render_pass.set_bind_group(1, bind_group_1, &[]);

        // Send vertices and indices
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // .. to use the entire buffer
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // .. to use the entire buffer

        let num_indices = Vertex::rectangle_indices().len() as u32;
        render_pass.draw_indexed(0..num_indices, 0, 0..1);

        Ok(())
    }

    fn on_resize(&mut self, new_width: f32, new_height: f32) {
        self.scene.on_resize(new_width, new_height);
        self.update_camera_buffer();
    }

    fn should_be_swapped(&self) -> bool {
        false
    }

    fn should_be_removed(&self) -> bool {
        false
    }
}
