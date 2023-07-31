use std::option::Option;

use crate::engine::math::ray::{*};
use crate::engine::material::{*};
use std::rc::{*};
use glam::{Vec2, Vec3A};

pub struct HitResult {
    pub position: Vec3A,
    pub t: f32,
    pub normal: Vec3A,
    pub material: Weak<Material>,
    pub uv: Vec2,
    pub front_face: bool
}

pub trait Traceable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

impl HitResult {
    pub fn new() -> Self {
        Self {
            position: Vec3A::new(0.0, 0.0, 0.0),
            t: f32::MAX,
            normal: Vec3A::new(0.0, 0.0, 0.0),
            material: Weak::new(),
            uv: Vec2::ZERO,
            front_face: true,
        }
    }
}