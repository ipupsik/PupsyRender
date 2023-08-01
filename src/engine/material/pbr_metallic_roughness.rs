use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use crate::engine::texture::texture2d::*;

use glam::{Vec3A, Vec4};

pub struct PBRMetallicRoughnessMaterial {
    pub base_color_factor: Vec4,
    pub base_color_texture: Option<Arc<Texture2D>>,
    pub metalic_roughness_texture: Option<Arc<Texture2D>>,
    pub metalic_factor: f32,
    pub roughness_factor: f32,
}

impl PBRMetallicRoughnessMaterial {
    pub fn new() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            base_color_texture: None,
            metalic_roughness_texture: None,
            metalic_factor: 0.0,
            roughness_factor: 0.0
        }
    }
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for PBRMetallicRoughnessMaterial {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<Vec3A> {
        let direction = reflect(ray.direction, hit_result.normal) + random_in_unit_sphere();
        Some(direction.normalize())
    }

    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        Vec3A::ONE
    }
}