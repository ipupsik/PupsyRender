use std::option::Option;

use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::material::{*};
use std::rc::{*};

pub struct HitResult {
    pub position: Vector3,
    pub t: f64,
    pub normal: Vector3,
    pub material: Weak<Material>,
}

pub trait Traceable {
    fn hit(&self, ray: Ray) -> Option<HitResult>;
}

impl HitResult {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            t: f64::MAX,
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Weak::new(),
        }
    }
}