use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;

#[derive(Copy, Clone)]
pub struct UVMaterial {
    
}

impl Material for UVMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        let diffuse_position = hit_result.normal + random_in_unit_sphere();
        diffuse_position
    }

    fn sample(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::new(hit_result.uvs[0].x, hit_result.uvs[0].y, 1.0 - hit_result.uvs[0].x - hit_result.uvs[0].y)
    }
}
