use crate::engine::{math::ray::*, material::Material};
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use super::aabb::*;
use rand::{Rng};


#[derive(Copy, Clone)]
pub struct Node {
    pub aabb: AABB,
    pub left_node_or_primitive_index: usize,
    pub primitives_count: usize,
}

impl Node {
    pub fn new() -> Self {
        Self {
            aabb: AABB::new(Vec3A::ZERO, Vec3A::ZERO),
            left_node_or_primitive_index: 0,
            primitives_count: 0
        }   
    }

    pub fn is_leaf(&self) -> bool {
        self.primitives_count > 0
    }
}