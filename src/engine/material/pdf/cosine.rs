use crate::engine::onb::*;
use crate::engine::math::utils::*;
use glam::{Vec2, Vec3A, Vec4};

use super::*;

pub struct CosinePDF {
    base_pdf: PDFBase,
}

impl PDF for CosinePDF {
    fn new(forward: Vec3A) -> Self {
        Self {
            base_pdf: PDFBase { basis: ONB::build_from_z(forward) }
        }
    }

    fn value(&self, direction: Vec3A) -> f32 {
        let cosine = self.base_pdf.basis.z.dot(direction);
        let scattered_pdf =  if cosine < 0.0 {0.0} else {cosine / std::f32::consts::PI};

        scattered_pdf
    }

    fn generate(&self) -> Vec3A {
        let scattering_direction = self.base_pdf.basis.get_position(random_cosine_direction()).normalize();
        scattering_direction
    }
}