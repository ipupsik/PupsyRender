use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::geometry::vertex::*;
use glam::{Vec2, Vec3A};
use std::mem;
use std::sync::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A
}

impl AABB {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self {
            min: min, 
            max: max,
        }
    }

    pub fn extend(&self, other_aabb: &AABB) -> Self {
        AABB::new(
            self.min.min(other_aabb.min),
            self.max.max(other_aabb.max),
        )
    }
}

impl Traceable for AABB {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min {t0} else {t_min};
            let t_max = if t1 < t_max {t1} else {t_max};
            if t_max <= t_min {
                return None;
            }
        }
        return Some(HitResult::new());
    }

    fn bounding_box(&self) -> AABB {
        *self
    }
}