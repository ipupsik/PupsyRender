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
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> (Vec3A, Option<Rc<dyn PDF>>) {
        let (_, scattering_direction) = self.diffuse.scatter(&ray, &hit_result);

        let sample = Vec3A::new(hit_result.uvs[0].x, hit_result.uvs[0].y, 1.0 - hit_result.uvs[0].x - hit_result.uvs[0].y);
        (sample, scattering_direction)
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        self.diffuse.scattering_pdf(&ray, &hit_result, &scattering)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}
