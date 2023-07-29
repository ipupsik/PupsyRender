use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};

pub struct Camera {
    pub aspect_ratio: f64,
    pub width: f64,
    pub height: f64,
    pub focal_length: f64,
    pub ray: Ray
}

impl Camera {
    pub fn get_ray(&self, u : f64, v : f64) -> Ray {
        let mut pixel_position = Vector3::new(0.0, 0.0, 0.0);
        pixel_position.vec[0] = self.width * (u - 1.0 / 2.0);
        pixel_position.vec[1] = self.height * (v - 1.0 / 2.0);
        pixel_position.vec[2] = self.focal_length;
        Ray{origin : self.ray.origin, direction : (pixel_position - self.ray.origin).normalize()}
    }
}