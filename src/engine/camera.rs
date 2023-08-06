use std::str::FromStr;

use crate::engine::math::ray::*;
use glam::{Vec3A, Vec4, Mat4};
use crate::engine::transform::*;
use crate::engine::onb::*;

pub trait Camera {
    fn get_ray(&self, u : f32, v : f32) -> Ray;
    fn aspect_ratio(&self) -> f32;
    fn name(&self) -> String;
}

pub struct CommonCamera {
    pub aspect_ratio: f32,
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
    pub transform: Transform,
    pub name: String,
}

pub struct PerspectiveCamera {
    pub camera: CommonCamera,
}

pub struct OrthographicCamera {
    pub camera: CommonCamera,
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

        let transform = Transform{
            basis: ONB{x : right, y: up, z: forward},
            translation: origin,
            scale: Vec3A::ONE,
            model_matrix: *transform,
        };

        Self {
            camera: CommonCamera{ aspect_ratio: aspect_ratio,
                width: width,
                height: height,
                focal_length: 1.0,
                transform: transform,
                name: String::from_str(name).unwrap() }
        }
    }
}

impl Camera for PerspectiveCamera{
    fn get_ray(&self, u : f32, v : f32) -> Ray {
        let pixel_position = self.camera.transform.basis.get_position(Vec3A::new(
            self.camera.width * (u - 1.0 / 2.0), 
            self.camera.height * (v - 1.0 / 2.0), 
            self.camera.focal_length));
        Ray{
            origin : self.camera.transform.translation, 
            direction : pixel_position.normalize()
        }
    }

    fn aspect_ratio(&self) -> f32 {
        self.camera.aspect_ratio
    }

    fn name(&self) -> String {
        format!("{}_Perspective", self.camera.name.as_str())
    }
}