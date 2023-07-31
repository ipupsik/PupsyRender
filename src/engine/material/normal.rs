use crate::engine::material::{*};

use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::math::utils::{*};

#[derive(Copy, Clone)]
pub struct NormalMaterial {
    
}

impl Scatter for NormalMaterial {
    fn scatter(&self, ray: Ray, hit_result : &HitResult) -> Option<Vec3A> {
        let diffuse_position = hit_result.normal + random_in_unit_sphere();
        Some(diffuse_position.normalize())
    }
}

impl Sample for NormalMaterial {
    fn sample(&self, hit_result : &HitResult) -> Vec3A {
       0.5 * (hit_result.normal + Vec3A::ONE)
    }
}