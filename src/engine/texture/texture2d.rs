use crate::engine::sampler::sampler::*;
use crate::engine::texture::*;
use glam::{Vec2, Vec3A, Vec4};
use image::GenericImageView;

pub struct Texture2D {
    pub texture: Texture
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

    pub fn valid(&self) -> bool {
        return self.texture.dimensions.len() == 2
    }

    pub fn sample(&self, sampler : &Sampler, uv: Vec2) -> Vec4 {
        let uv = uv.fract();

        if !self.valid() {
            return Vec4::ONE;
        }

        let x = uv.x * (self.texture.dimensions[0] - 1) as f32;
        let y = uv.y * (self.texture.dimensions[1] - 1) as f32;

        let color = self.texture.raw_texture.get_pixel(x as u32, y as u32);

        let final_color = Vec4::new(color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32);

        final_color / (256.0 - 1.0)
    }
}