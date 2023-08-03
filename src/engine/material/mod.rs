pub mod diffuse;
pub mod metal;
pub mod normal;
pub mod uv;
pub mod refraction;
pub mod pbr;
pub mod pbr_metallic_roughness;

use crate::engine::math::ray::*;
use glam::{Vec3A};
use crate::engine::material::diffuse::*;
use crate::engine::material::metal::*;
use crate::engine::geometry::traceable::*;

use std::rc::*;
use std::sync::{Arc};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A;
    fn sample(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A;
}
