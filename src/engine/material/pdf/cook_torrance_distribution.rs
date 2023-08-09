use crate::engine::onb::*;
use crate::engine::math::utils::*;
use glam::{Vec2, Vec3A, Vec4};

use super::*;

#[derive(Copy, Clone)]
pub struct CookTorranceDistributionPDF {
    pub base_pdf: PDFBase,
    pub generated_direction: Vec3A,
    pub pdf: f32,
}

impl CookTorranceDistributionPDF {
    pub fn new(forward: Vec3A) -> Self {
        Self {
            base_pdf: PDFBase { basis: ONB::build_from_z(forward) },
            generated_direction: Vec3A::ZERO,
            pdf: 1.0
        }
    }
}

impl PDF for CookTorranceDistributionPDF {
    fn value(&self, direction: Vec3A) -> f32 {
        self.pdf
    }

    fn generate(&self) -> Vec3A {
       self.generated_direction
    }
}