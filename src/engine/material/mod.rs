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

use std::{sync::{Arc}, rc::Rc};

use self::{pdf::PDF, diffuse::DiffuseMaterial};

pub struct ScatterResult {
    pub attenuation: Vec3A,
    pub scatter: Option<Rc<dyn PDF>> /* Scatter */,
    pub alpha_masked: bool,
    pub hit_result: HitResult,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> ScatterResult;
    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A;
}
