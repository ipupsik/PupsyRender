use std::str::FromStr;

use crate::engine::math::ray::*;
use glam::{Vec3A, Vec4, Mat4};

#[derive(Copy, Clone)]
pub struct ONB {
    pub x: Vec3A,
    pub y: Vec3A,
    pub z: Vec3A
}

impl ONB {
    pub fn build_from_z(z: Vec3A) -> Self {
        let a = if z.x.abs() > 0.9 {Vec3A::new(0.0, 1.0, 0.0)} else {Vec3A::new(1.0, 0.0, 0.0)};
        let y = z.cross(a).normalize();
        let x = z.cross(y);
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn get_position(&self, point: Vec3A) -> Vec3A {
        point.x * self.x + point.y  * self.y + point.z * self.z
    }
}