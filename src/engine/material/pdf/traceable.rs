use std::sync::Arc;

use crate::engine::math::ray::Ray;
use crate::engine::{onb::*, geometry::traceable::Mesh};
use crate::engine::math::utils::*;
use glam::{Vec2, Vec3A, Vec4};

use super::*;

pub struct GeometryPDF {
    pub geometry: Arc<dyn Mesh>,
    pub origin: Vec3A
}

impl GeometryPDF {
    
}

impl PDF for GeometryPDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let ray = Ray{origin : self.origin, direction : direction};
        self.geometry.pdf(&ray, 0.001, f32::MAX)
    }

    fn generate(&self) -> Vec3A {
        (self.geometry.random() - self.origin).normalize()
    }
}