pub mod brdf;
pub mod diffuse;

use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::material::diffuse::{*};
use crate::engine::geometry::traceable::{*};

pub enum MaterialType {
    Diffuse
}

pub struct Material {
    material_type : MaterialType,
}

impl Material {
    pub fn new(material_type : MaterialType) -> Self {
        Self{material_type : material_type}
    }

    pub fn brdf(&self, ray: Ray, hit_result : &HitResult) -> Vector3 {
        match self.material_type {
            MaterialType::Diffuse => DiffuseMaterial::brdf(self, ray, hit_result),
        }
    }
}