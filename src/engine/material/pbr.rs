use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use super::pbr_metallic_roughness::*;
use crate::engine::texture::texture2d::*;
use crate::engine::sampler::*;
use crate::engine::texture::*;

use glam::{Vec3A};

pub struct PBRMaterial {
    pub pbr_metallic_roughness: PBRMetallicRoughnessMaterial,

    pub normal_texture: Arc<Texture2D>,
    pub normal_texture_sampler: Sampler,

    pub occlusion_texture: Arc<Texture2D>,
    pub occlusion_texture_sampler: Sampler,

    pub emissive_texture: Arc<Texture2D>,
    pub emissive_texture_sampler: Sampler,
    pub emissive_factor: Vec3A,
}

impl PBRMaterial {
    pub fn new() -> Self {
        Self {
            pbr_metallic_roughness: PBRMetallicRoughnessMaterial::new(),
            normal_texture:  Arc::new(Texture2D::new(Texture::new(0, 0, Arc::new(Vec::new())))),
            normal_texture_sampler: Sampler::new(),
            occlusion_texture:  Arc::new(Texture2D::new(Texture::new(0, 0, Arc::new(Vec::new())))),
            occlusion_texture_sampler: Sampler::new(),
            emissive_texture:  Arc::new(Texture2D::new(Texture::new(0, 0, Arc::new(Vec::new())))),
            emissive_texture_sampler: Sampler::new(),
            emissive_factor: Vec3A::ZERO,
        }
    }
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for PBRMaterial {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<Vec3A> {
        let diffuse_position = hit_result.normal + random_in_unit_sphere();
        Some(diffuse_position.normalize())
    }

    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        return self.pbr_metallic_roughness.sample(hit_result);
    }
}