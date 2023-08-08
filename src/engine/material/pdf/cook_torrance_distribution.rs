use crate::engine::onb::*;
use crate::engine::math::utils::*;
use glam::{Vec2, Vec3A, Vec4};

use super::*;

#[derive(Copy, Clone)]
pub struct CookTorranceDistributionPDF {
    base_pdf: PDFBase,
    roughness: f32
}

impl CookTorranceDistributionPDF {
    pub fn new(forward: Vec3A, roughness: f32) -> Self {
        Self {
            base_pdf: PDFBase { basis: ONB::build_from_z(forward) },
            roughness: roughness
        }
    }
}

impl PDF for CookTorranceDistributionPDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine = self.base_pdf.basis.z.dot(direction);
        let scattered_pdf =  if cosine < 0.0 {0.0} else {cosine / std::f32::consts::PI};

        scattered_pdf
    }

    fn generate(&self) -> Vec3A {
        let scattering_direction = self.base_pdf.basis.get_position(
            random_ggx_hemisphere_direction(self.roughness * self.roughness)
        ).normalize();
        scattering_direction
    }
}