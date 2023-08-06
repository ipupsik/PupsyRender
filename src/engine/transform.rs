use std::str::FromStr;

use crate::engine::math::ray::*;
use crate::engine::onb::*;
use glam::{Vec3A, Vec4, Mat4};

pub struct Transform {
    pub basis: ONB,
    pub translation: Vec3A,
    pub scale: Vec3A,
    pub model_matrix: Mat4,
}