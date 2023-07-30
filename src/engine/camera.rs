use crate::engine::math::ray::{*};
use glam::{Vec3A};

pub struct Camera {
    pub aspect_ratio: f32,
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
    pub ray: Ray
}

impl Camera {
    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        let mut pixel_position = Vec3A::new(0.0, 0.0, 0.0);
        pixel_position.x = self.width * (u - 1.0 / 2.0);
        pixel_position.y = self.height * (v - 1.0 / 2.0);
        pixel_position.z = self.focal_length;
        Ray{origin : self.ray.origin, direction : (pixel_position - self.ray.origin).normalize()}
    }
}