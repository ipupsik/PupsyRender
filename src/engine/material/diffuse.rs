use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;
use crate::engine::onb::*;

use super::pdf::cosine::CosinePDF;

pub struct DiffuseMaterial {
    
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult, light_scattering: &Ray) -> ScatterResult {
        ScatterResult{
            attenuation: Vec3A::ONE, 
            scatter: Some(Rc::new(CosinePDF::new(hit_result.normal))),
            alpha_masked: false,
            hit_result: hit_result.clone()
        }
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        let cosine = hit_result.normal.dot(scattering.direction);
        let scattered_pdf =  if cosine < 0.0 {0.0} else {cosine / std::f32::consts::PI};
        scattered_pdf
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}