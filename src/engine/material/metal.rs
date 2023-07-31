use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use glam::{Vec3A};

#[derive(Copy, Clone)]
pub struct MetalMaterial {
    pub metalness: f64,
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Scatter for MetalMaterial {
    fn scatter(&self, ray: Ray, hit_result: &HitResult) -> Option<Vec3A> {
        Some(reflect(ray.direction, hit_result.normal))
    }
}

impl Sample for MetalMaterial {
    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        Vec3A::ONE
    }
}