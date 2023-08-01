use crate::engine::sampler::*;
use std::sync::*;
use glam::{Vec2, Vec4};

pub struct Texture2D {
    buffer: Arc<Vec<u8>>
}

impl Texture2D {
    pub fn new(buffer: Arc<Vec<u8>>) -> Self {
        Self {
            buffer: buffer.clone(),
        }
    }

    pub fn sample(&self, sampler : &Sampler, uv: Vec2) -> Vec4 {
        Vec4::ZERO
    }
}