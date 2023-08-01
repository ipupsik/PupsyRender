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
        let index = (uv.y * self.texture.height as f32 * self.texture.width as f32 + uv.x * self.texture.width as f32) * 3.0;
        Vec4::from((Vec3A::new(
            self.texture.buffer[index as usize] as f32 / 256.0, 
            self.texture.buffer[index as usize + 1] as f32 / 256.0,
            self.texture.buffer[index as usize + 2] as f32 / 256.0
        ), 1.0))
    }
}