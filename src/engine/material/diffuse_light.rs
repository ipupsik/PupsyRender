use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;

pub struct DiffuseLightMaterial {
    pub color: Vec3A,
}

impl Material for DiffuseLightMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> (Vec3A, Option<Vec3A>, f32) {
        (Vec3A::ONE, None, 1.0)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        self.color
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattered_direction: Vec3A) -> f32 {
        let cosine = hit_result.normal.dot(scattered_direction);
        return if cosine < 0.0 {0.0} else {cosine / std::f32::consts::PI};
    }
}