use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use glam::{Vec3A};

use super::pdf::cosine::CosinePDF;

pub struct MetalMaterial {
    pub metalness: f32,
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult, light_scattering: &Option<Ray>) -> ScatterResult {
        let direction = reflect(ray.direction, hit_result.normal) + (1.0 - self.metalness) * random_in_unit_sphere();
        
        ScatterResult{
            attenuation: Vec3A::ONE, 
            scatter: Some(Rc::new(CosinePDF::new(direction.normalize()))),
            alpha_masked: false,
            hit_result: hit_result.clone()
        }
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        1.0
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}