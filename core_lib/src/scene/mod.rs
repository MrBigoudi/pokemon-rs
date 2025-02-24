use camera::Camera;

pub mod animation;
pub mod camera;
pub mod geometry;
pub mod rendering;
pub mod text;

pub struct Scene {
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            camera: Camera::new(width, height),
        }
    }

    pub fn on_resize(&mut self, new_width: f32, new_height: f32) {
        self.camera.aspect_ratio = new_width / new_height;
    }
}
