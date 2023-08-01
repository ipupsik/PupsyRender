use std::option::Option;

use crate::engine::{math::ray::*, material::diffuse::DiffuseMaterial};
use crate::engine::material::*;
use std::sync::*;
use crate::engine::bvh::aabb::*;
use glam::{Vec2, Vec3A};

pub trait Traceable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
    fn bounding_box(&self) -> AABB;
}

pub struct HitResult {
    pub position: Vec3A,
    pub t: f32,
    pub normal: Vec3A,
    pub uv: Vec2,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitResult {
    pub fn new() -> Self {
        Self {
            position: Vec3A::new(0.0, 0.0, 0.0),
            t: f32::MAX,
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::ZERO,
            front_face: true,
            material: Arc::new(DiffuseMaterial{})
        }
    }
}