use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use std::cmp::Ordering;

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

    pub fn cmp<'a>(
        a: &'a Arc<dyn Traceable>,
        b: &'a Arc<dyn Traceable>,
        axis: usize,
    ) -> Ordering {
        let mut box_a = AABB::new(Vec3A::ZERO, Vec3A::ZERO);
        let mut box_b = AABB::new(Vec3A::ZERO, Vec3A::ZERO);

        let a = box_a.min[axis];
        let b = box_b.min[axis];
        a.partial_cmp(&b).unwrap()
    }
}

impl Traceable for AABB {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        let t_min = (self.min - ray.origin) / ray.direction;
        let t_max = (self.max - ray.origin) / ray.direction;
        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);
        let t_near = t1.max_element();
        let t_far = t2.min_element();

        if t_near <= t_far && t_far >= 0.0 {
            return Some(HitResult::new());
        }
        return None;
    }

    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        0.0
    }

    fn random(&self) -> Vec3A {
        Vec3A::ZERO
    }

    fn bounding_box(&self) -> AABB {
        *self
    }
}