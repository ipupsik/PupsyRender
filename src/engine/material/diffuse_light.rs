use std::rc::Rc;

use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;

use super::pdf::PDF;

pub struct DiffuseLightMaterial {
    pub color: Vec3A,
}

impl Material for DiffuseLightMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> ScatterResult {
        ScatterResult{
            attenuation: Vec3A::ONE, 
            scatter: None,
            alpha_masked: false,
            hit_result: hit_result.clone()
        }
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        self.color
    }
}