use crate::engine::sampler::*;
use crate::engine::texture::*;
use std::sync::*;
use glam::{Vec2, Vec3A, Vec4};

pub struct Texture2D {
    texture: Texture
}

impl Texture2D {
    pub fn null() -> Self {
        Self {
            texture: Texture::null(),
        }
    }

    pub fn new(texture: Texture) -> Self {
        Self {
            texture: texture,
        }
    }

    pub fn sample(&self, sampler : &Sampler, uv: Vec2) -> Vec4 {
        uv.clamp(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));

        let mut x = uv.x * (self.texture.dimensions[0] - 1) as f32;
        let mut y = uv.y * (self.texture.dimensions[1] - 1) as f32;
        let mut index = y * self.texture.dimensions[0] as f32 + x;
        index *= self.texture.bytes_per_component as f32 * self.texture.components_per_pixel as f32;

        assert!(self.texture.components_per_pixel == 3);

        if self.texture.components_per_pixel == 3 {
            return Vec4::from((Vec3A::new(
                self.texture.buffer[index as usize] as f32 / 256.0, 
                self.texture.buffer[index as usize + 1] as f32 / 256.0,
                self.texture.buffer[index as usize + 2] as f32 / 256.0
            ), 1.0));
        }

        Vec4::ZERO
    }
}