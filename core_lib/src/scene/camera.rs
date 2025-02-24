use crate::utils::time::Duration;

use super::animation::movement::MovementDirection;

#[derive(Debug, Default)]
pub enum ProjectionType {
    #[default]
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect_ratio: f32,
    pub fov_y_radians: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub speed: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let eye = glam::Vec3::new(0., 1., 2.);
        let target = glam::Vec3::new(0., 0., 0.);
        let up = glam::Vec3::new(0., 1., 0.);

        let fov_y_radians = 45_f32.to_radians();
        let z_near = 0.;
        let z_far = 100.;

        let aspect_ratio = width / height;

        let speed = 0.001;

        Camera {
            eye,
            target,
            up,
            aspect_ratio,
            fov_y_radians,
            z_near,
            z_far,
            speed,
        }
    }

    pub fn get_view(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.eye, self.target, self.up)
    }

    pub fn get_perspective(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(
            self.fov_y_radians,
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        )
    }

    pub fn to_camera_gpu(&self, projection_type: ProjectionType) -> CameraGPU {
        let view = self.get_view();
        let proj = match projection_type {
            ProjectionType::Perspective => self.get_perspective(),
            ProjectionType::Orthographic => todo!("Implement orthographic projections"),
        };
        CameraGPU { view, proj }
    }

    pub fn on_move(&mut self, movement: MovementDirection, delta_time: Duration) {
        let forward_direction = (self.target - self.eye).normalize();
        let right_direction = forward_direction.cross(self.up).normalize();
        let speed = self.speed * delta_time as f32;
        match movement {
            MovementDirection::Forward => {
                self.eye += speed * forward_direction;
                self.target += speed * forward_direction;
            }
            MovementDirection::Backward => {
                self.eye -= speed * forward_direction;
                self.target -= speed * forward_direction;
            }
            MovementDirection::Left => {
                self.eye -= speed * right_direction;
                self.target -= speed * right_direction;
            }
            MovementDirection::Right => {
                self.eye += speed * right_direction;
                self.target += speed * right_direction;
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraGPU {
    pub view: glam::Mat4,
    pub proj: glam::Mat4,
}

unsafe impl bytemuck::Pod for CameraGPU {}
unsafe impl bytemuck::Zeroable for CameraGPU {}
