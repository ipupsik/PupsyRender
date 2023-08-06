use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;
use crate::engine::onb::*;

pub struct DiffuseMaterial {
    
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> (Vec3A, Option<Vec3A>, f32) {
        let onb = ONB::build_from_z(hit_result.normal);
        let scattering_direction = onb.get_position(random_cosine_direction());
        //let scattering_direction = onb.get_position(random_in_hemisphere(hit_result.normal));
        //let scattering_direction = random_in_hemisphere(hit_result.normal);
        let pdf = scattering_direction.dot(onb.z) / std::f32::consts::PI;
        (Vec3A::ONE, Some(scattering_direction.normalize()), pdf)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattered_direction: Vec3A) -> f32 {
        let cosine = hit_result.normal.dot(scattered_direction);
        return if cosine < 0.0 {0.0} else {cosine / std::f32::consts::PI};
    }
}