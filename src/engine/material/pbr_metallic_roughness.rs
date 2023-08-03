use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use crate::engine::texture::texture2d::*;
use crate::engine::sampler::sampler::*;
use crate::engine::texture::*;

use glam::{Vec3A, Vec4};

pub struct PBRMetallicRoughnessMaterial {
    pub base_color_factor: Vec4,

    pub base_color_texture: Arc<Texture2D>,
    pub base_color_texture_sampler: Sampler,

    pub metalic_roughness_texture: Arc<Texture2D>,
    pub metalic_roughness_texture_sampler: Sampler,
    pub metalic_factor: f32,
    pub roughness_factor: f32,
}

impl PBRMetallicRoughnessMaterial {
    pub fn new() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            base_color_texture: Arc::new(Texture2D::null()),
            base_color_texture_sampler: Sampler::new(),
            metalic_roughness_texture:  Arc::new(Texture2D::null()),
            metalic_roughness_texture_sampler: Sampler::new(),
            metalic_factor: 0.0,
            roughness_factor: 0.0
        }
    }
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for PBRMetallicRoughnessMaterial {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Vec3A {
        let diffuse_position = hit_result.normal + random_in_unit_sphere();
        diffuse_position.normalize()
    }

    fn sample(&self, hit_result : &HitResult) -> Vec3A {
        let mut sample = self.base_color_factor;
        sample = sample * self.base_color_texture.sample(
            &self.base_color_texture_sampler, 
            self.base_color_texture.texture.get_uv_by_index(&hit_result.uvs)
        );
        return Vec3A::from(sample);
    }
}