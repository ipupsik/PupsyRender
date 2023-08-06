use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;
use crate::engine::onb::*;

use super::diffuse::DiffuseMaterial;

pub struct NormalMaterial {
    pub diffuse: DiffuseMaterial,
}

impl Material for NormalMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> (Vec3A, Option<Vec3A>, f32) {
        let (_, scattering_direction, pdf) = self.diffuse.scatter(&ray, &hit_result);
        let sample =  0.5 * (hit_result.normal + Vec3A::ONE);
        (sample, scattering_direction, pdf)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattered_direction: Vec3A) -> f32 {
        return self.diffuse.scattering_pdf(&ray, &hit_result, scattered_direction);
    }
}