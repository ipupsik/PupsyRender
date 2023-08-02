use crate::engine::sampler::*;
use crate::engine::texture::*;
use std::sync::*;
use glam::{Vec2, Vec3A, Vec4};

pub struct Texture2D {
    texture: Texture
}

impl Texture2D {
    pub fn new(texture: Texture) -> Self {
        Self {
            texture: texture,
        }
    }

    pub fn sample(&self, sampler : &Sampler, uv: Vec2) -> Vec4 {
        Vec4::ONE
    }
}