use crate::engine::material::{*};

use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::geometry::traceable::{*};

pub struct DiffuseMaterial {
    
}

impl Scatter for DiffuseMaterial {
    fn scatter(&self, ray: Ray, hit_result : &HitResult) -> Vector3 {
        let diffuse_position = hit_result.normal + Vector3::random_in_unit_sphere();
        diffuse_position.normalize()
    }
}