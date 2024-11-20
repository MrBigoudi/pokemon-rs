use camera::Camera;
use winit::dpi::PhysicalSize;

pub mod camera;
pub mod geometry;
pub mod rendering;

pub struct Scene {
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            camera: Camera::new(width, height),
        }
    }

    pub fn on_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.camera.aspect_ratio = new_size.width as f32 / new_size.height as f32;
    }
}
