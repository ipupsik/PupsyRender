pub mod cosine;
pub mod mix;
pub mod traceable;

use crate::engine::onb::*;
use glam::{Vec2, Vec3A, Vec4};

#[derive(Copy, Clone)]
pub struct PDFBase {
    pub basis: ONB,

}

pub trait PDF {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}