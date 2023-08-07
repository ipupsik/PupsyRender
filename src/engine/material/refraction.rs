use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use glam::{Vec3A};

use super::pdf::cosine::CosinePDF;

#[derive(Copy, Clone)]
pub enum RefractionType {
    Air,
    Glass,
    Diamond,
}

pub struct RefractionMaterial {
    pub refraction_type: RefractionType,
}

fn ior(refraction_type: RefractionType, front_face: bool) -> f32 {
    let ior = match refraction_type {
        RefractionType::Air => 1.0,
        RefractionType::Glass => 1.3,
        RefractionType::Diamond => 2.4,
        _ => 1.0
    };

    if front_face {
        return 1.0 / ior;
    }
    return ior;
}


fn refract(ray: &Ray, hit_result: &HitResult, ior: f32) -> Vec3A {
    let cos_theta = (-1.0 * ray.direction).dot(hit_result.normal);
    let r_out_perp =  ior * (ray.direction + cos_theta * hit_result.normal);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * hit_result.normal;
    return r_out_perp + r_out_parallel;
}

impl Material for RefractionMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> ScatterResult {
        let ior = ior(self.refraction_type, hit_result.front_face);

        let direction = refract(ray, hit_result, ior).normalize();
        
        ScatterResult{
            attenuation: Vec3A::ONE, 
            scatter: Some(Rc::new(CosinePDF::new(direction))),
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