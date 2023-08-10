use crate::engine::material::Material;
use crate::engine::material::diffuse::DiffuseMaterial;
use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A,
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

    pub fn area(&self) -> f32 {
        let extent = self.max - self.min;
        return extent.x * extent.y + extent.x * extent.z + extent.y * extent.z;
    }

    pub fn cmp<'a>(
        a: &'a Arc<dyn Traceable>,
        b: &'a Arc<dyn Traceable>,
        axis: usize,
    ) -> Ordering {
        let a = a.bounding_box().min[axis];
        let b = b.bounding_box().min[axis];
        a.partial_cmp(&b).unwrap()
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let aabb_t_min = (self.min - ray.origin) / ray.direction;
        let aabb_t_max = (self.max - ray.origin) / ray.direction;
        let t1 = aabb_t_min.min(aabb_t_max);
        let t2 = aabb_t_min.max(aabb_t_max);
        let t_near = t1.max_element().max(t_min);
        let t_far = t2.min_element().min(t_max);

        if t_near <= t_far && t_far >= 0.0 {
            return Some(t_near);
        }
        return None;
    }
}