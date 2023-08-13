use crate::engine::material::Material;
use crate::engine::material::diffuse::DiffuseMaterial;
use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use std::cmp::Ordering;

use super::aabb::AABB;

#[derive(Copy, Clone)]
pub struct Bin {
    pub aabb: AABB,
    pub primitive_count: usize,
}

impl Bin {
    pub fn new() -> Self {
        Self {
            aabb: AABB::new(Vec3A::MAX, Vec3A::MIN),
            primitive_count: 0
        }
    }
}