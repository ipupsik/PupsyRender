use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::vector3::*;

pub struct MetalMaterial {
    pub metalness: f64,
}

pub fn reflect(eye: Vector3, normal: Vector3) -> Vector3 {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Scatter for MetalMaterial {
    fn scatter(&self, ray: Ray, hit_result: &HitResult) -> Vector3 {
        reflect(ray.direction, hit_result.normal)
    }
}
