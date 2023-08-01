use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use glam::{Vec3A};

pub struct MetalMaterial {
    pub metalness: f32,
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<Vec3A> {
        let direction = reflect(ray.direction, hit_result.normal) + (1.0 - self.metalness) * random_in_unit_sphere();
        Some(direction.normalize())
    }

    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        Vec3A::ONE
    }
}