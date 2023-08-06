use crate::engine::onb::*;
use crate::engine::math::utils::*;
use glam::{Vec2, Vec3A, Vec4};
use std::{sync::{Arc}, rc::Rc};
use rand::{Rng};

use super::*;

pub struct MixPDF {
    pub pdfs: Vec<Rc<dyn PDF>>,
    pub weights: Vec<f32>
}

impl PDF for MixPDF {
    fn value(&self, direction: Vec3A) -> f32 {
        assert!(self.pdfs.len() == self.weights.len());
        let mut pdf_value = 0.0;
        for (index, pdf) in self.pdfs.iter().enumerate()  {
            pdf_value += self.weights[index] * pdf.value(direction);
        }

        pdf_value
    }

    fn generate(&self) -> Vec3A {
        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);

        let mut acc_weight = 0.0;
        for (index, pdf) in self.pdfs.iter().enumerate()  {
            acc_weight += self.weights[index];
            if random <= acc_weight {
                return pdf.generate();
            }
        }
    
        return Vec3A::ZERO;
    }
}