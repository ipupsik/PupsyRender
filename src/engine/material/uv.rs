use crate::engine::material::*;

use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::math::utils::*;
use crate::engine::onb::*;
use super::diffuse::*;

pub struct UVMaterial {
    pub diffuse: DiffuseMaterial,
}

impl Material for UVMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult, light_scattering: &Ray) -> ScatterResult {
        let mut scatter_result = self.diffuse.scatter(&ray, &hit_result, &light_scattering);

        let sample = Vec3A::new(hit_result.uvs[0].x, hit_result.uvs[0].y, 1.0 - hit_result.uvs[0].x - hit_result.uvs[0].y);
        scatter_result.attenuation = sample;

        scatter_result
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        self.diffuse.scattering_pdf(&ray, hit_result, &scattering)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}
