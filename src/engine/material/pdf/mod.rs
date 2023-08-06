pub mod cosine;

use crate::engine::onb::*;
use glam::{Vec2, Vec3A, Vec4};

pub struct PDFBase {
    pub basis: ONB,

}

pub trait PDF {
    fn new(forward: Vec3A) -> Self;
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}