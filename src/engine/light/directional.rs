use crate::engine::transform;
use crate::engine::{material::*, transform::Transform};

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::texture::texture2d::*;
use crate::engine::sampler::sampler::*;
use crate::engine::material::pdf::cook_torrance_distribution::*;

use glam::{Vec2, Vec3A, Vec4};

pub struct DirectionalLight {
    pub color: Vec3A,
    pub intensity: f32,
    pub light_vector: Vec3A,
}

impl DirectionalLight {
    pub fn new(color: Vec3A, intensity: f32, light_vector: Vec3A) -> Self {
        Self {
            color: color,
            intensity: intensity,
            light_vector: light_vector,
        }
    }
}