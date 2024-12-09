use std::path::PathBuf;

use core_lib::scene::{animation::movement::MovementDirection, geometry::vertex::Vertex, rendering::texture::Texture};

pub struct PlayerSprite {
    pub width: f32,
    pub height: f32,
    pub left: f32,
    pub top: f32,
}

#[derive(Debug)]
pub enum PlayerState {
    Down(u8), 
    Up(u8), 
    Left(u8), 
    Right(u8),
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Down(0)
    }
}

impl PlayerSprite {
    pub fn from_state(state: &PlayerState) -> Self {
        let width = 64.;
        let height = 64.;
        match state {
            PlayerState::Down(index) => {
                let left = *index as f32 * width;
                PlayerSprite {
                    width, height, left, top:0.,
                }
            },
            PlayerState::Left(index) => {
                let left = *index as f32 * width;
                PlayerSprite {
                    width, height, left, top:64.,
                }
            },
            PlayerState::Right(index) => {
                let left = *index as f32 * width;
                PlayerSprite {
                    width, height, left, top:128.,
                }
            },
            PlayerState::Up(index) => {
                let left = *index as f32 * width;
                PlayerSprite {
                    width, height, left, top:192.,
                }
            },
        }
    }
}

pub struct Player {
    pub state: PlayerState,
    pub texture_width: u32,
    pub texture_height: u32,
    pub nb_steps_before_switch: u8,
    pub nb_steps: u8,
}

impl Player {
    pub fn get_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let mut diffuse_texture_path = PathBuf::from("");
        diffuse_texture_path.push("assets");
        diffuse_texture_path.push("sprites");
        diffuse_texture_path.push("characters");
        diffuse_texture_path.push("player");
        diffuse_texture_path.push("player");
        diffuse_texture_path.set_extension("png");

        pollster::block_on(Texture::from_path(
            &diffuse_texture_path,
            device,
            queue,
            None,
        ))
        .unwrap()
    }

    pub fn new() -> Self {
        let state = PlayerState::default();
        Self {
            state,
            texture_width: 256,
            texture_height: 256,
            nb_steps_before_switch: 30,
            nb_steps: 0,
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let texture_width = self.texture_width as f32;
        let texture_height = self.texture_height as f32;

        let sprite = PlayerSprite::from_state(&self.state);
        let tex_coord_left = sprite.left / texture_width;
        let tex_coord_top = sprite.top / texture_height;
        let tex_coord_width = sprite.width / texture_width;
        let tex_coord_height = sprite.height / texture_height;

        let tex_top_left = glam::Vec2::new(tex_coord_left, tex_coord_top);
        let tex_bottom_left = glam::Vec2::new(tex_coord_left, tex_coord_top+tex_coord_height);
        let tex_top_right = glam::Vec2::new(tex_coord_left+tex_coord_width, tex_coord_top);
        let tex_bottom_right = glam::Vec2::new(tex_coord_left+tex_coord_width, tex_coord_top+tex_coord_height);

        let mut vertices = Vertex::rectangle_vertices();
        vertices[0].tex_coords = tex_top_left;
        vertices[1].tex_coords = tex_bottom_left;
        vertices[2].tex_coords = tex_top_right;
        vertices[3].tex_coords = tex_bottom_right;

        vertices
    }

    pub fn get_indices(&self) -> Vec<u16> {
        Vertex::rectangle_indices()
    }

    pub fn on_move(&mut self, movement: MovementDirection){
        self.nb_steps += 1;
        let should_update_step = (self.nb_steps % self.nb_steps_before_switch) == 0;

        match movement {
            MovementDirection::Forward => {
                if let PlayerState::Up(step) = self.state {
                    if should_update_step {
                        self.state = PlayerState::Up((step+1)%4);
                    }
                } else {
                    self.state = PlayerState::Up(0);
                }
            },
            MovementDirection::Backward => {
                if let PlayerState::Down(step) = self.state {
                    if should_update_step {
                        self.state = PlayerState::Down((step+1)%4);
                    }
                } else {
                    self.state = PlayerState::Down(0);
                }
            },
            MovementDirection::Left => {
                if let PlayerState::Left(step) = self.state {
                    if should_update_step {
                        self.state = PlayerState::Left((step+1)%4);
                    }
                } else {
                    self.state = PlayerState::Left(0);
                }
            },
            MovementDirection::Right => {
                if let PlayerState::Right(step) = self.state {
                    if should_update_step {
                        self.state = PlayerState::Right((step+1)%4);
                    }
                } else {
                    self.state = PlayerState::Right(0);
                }
            },
        }

        if should_update_step {
            self.nb_steps = 0;
        }
    }
}