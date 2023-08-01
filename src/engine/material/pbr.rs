use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use super::pbr_metallic_roughness::*;
use crate::engine::texture::texture2d::*;

use glam::{Vec3A};

pub struct PBRMaterial {
    pub pbr_metallic_roughness: PBRMetallicRoughnessMaterial,
    pub normal_texture: Option<Arc<Texture2D>>,
    pub occlusion_texture: Option<Arc<Texture2D>>,
    pub emissive_texture: Option<Arc<Texture2D>>,
    pub emissive_factor: Vec3A,
}

impl PBRMaterial {
    pub fn new() -> Self {
        Self {
            pbr_metallic_roughness: PBRMetallicRoughnessMaterial::new(),
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: Vec3A::ZERO,
        }
    }
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for PBRMaterial {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<Vec3A> {
        let direction = reflect(ray.direction, hit_result.normal) + random_in_unit_sphere();
        Some(direction.normalize())
    }

    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        Vec3A::ONE
    }
}