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
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> ScatterResult {
        let mut scatter_result = self.diffuse.scatter(&ray, &hit_result);
        let sample =  0.5 * (hit_result.normal + Vec3A::ONE);
        scatter_result.attenuation = sample;

        scatter_result
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        self.diffuse.scattering_pdf(&ray, &hit_result, &scattering)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}