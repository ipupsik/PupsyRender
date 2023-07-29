use crate::engine::material::{*};
use crate::engine::material::brdf::{*};

use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::geometry::traceable::{*};

pub trait DiffuseMaterial {
    fn brdf(&self, ray: Ray, hit_result : &HitResult) -> Vector3;
}

impl DiffuseMaterial for Material {
    fn brdf(&self, ray: Ray, hit_result : &HitResult) -> Vector3 {
        let diffuse_position = hit_result.normal + Vector3::random_in_unit_sphere();
        diffuse_position.normalize()
    }
}