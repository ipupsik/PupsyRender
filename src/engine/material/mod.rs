pub mod diffuse;
pub mod diffuse_light;
pub mod metal;
pub mod normal;
pub mod uv;
pub mod refraction;
pub mod pbr;
pub mod pbr_metallic_roughness;

pub mod pdf;

use crate::engine::math::ray::*;
use glam::{Vec3A};
use crate::engine::geometry::traceable::*;

use std::sync::{Arc};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> (
        Vec3A /* Attenuation */, 
        Option<Vec3A> /* Scatter */,
        f32 /* Scattered PDF */);
    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A;
}
