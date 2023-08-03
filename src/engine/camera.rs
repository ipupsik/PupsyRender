use std::str::FromStr;

use crate::engine::math::ray::*;
use glam::{Vec3A, Vec4, Mat4};

pub trait Camera {
    fn get_ray(&self, u : f32, v : f32) -> Ray;
}
pub struct PerspectiveCamera {
    pub aspect_ratio: f32,
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
    pub up: Vec3A,
    pub right: Vec3A,
    pub ray: Ray,
    pub name: String,
}

pub struct OrthographicCamera {
    pub aspect_ratio: f32,
    pub width: f32,
    pub height: f32,
    pub up: Vec3A,
    pub right: Vec3A,
    pub ray: Ray
}

impl PerspectiveCamera {
    pub fn new(transform: &Mat4,
        vertical_fov: f32, // vertical field-of-view in degrees
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
        name: &str,
    ) -> Self {

        let h = (vertical_fov / 2.0).tan();
        let height = 2.0 * h;
        let width = height * aspect_ratio;

        let forward = Vec3A::from(transform.mul_vec4(Vec4::new(0.0, 0.0, -1.0, 0.0))).normalize();
        let right = Vec3A::from(transform.mul_vec4(Vec4::new(1.0, 0.0, 0.0, 0.0))).normalize();
        let up = right.cross(forward).normalize();

        let origin = Vec3A::from(transform.mul_vec4(Vec4::new(0.0, 0.0, 0.0, 1.0)));

        Self {
            aspect_ratio: aspect_ratio,
            width: width,
            height: height,
            focal_length: 1.0,
            up: up,
            right: right,
            ray: Ray{origin: origin, direction: forward},
            name: String::from_str(name).unwrap()
        }
    }
}

impl Camera for PerspectiveCamera{
    fn get_ray(&self, u : f32, v : f32) -> Ray {
        let mut pixel_position = self.ray.origin;
        pixel_position += self.ray.direction * self.focal_length;
        pixel_position += self.right * self.width * (u - 1.0 / 2.0);
        pixel_position += self.up * self.height * (v - 1.0 / 2.0);
        Ray{origin : self.ray.origin, direction : (pixel_position - self.ray.origin).normalize()}
    }
}