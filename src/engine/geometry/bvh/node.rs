use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::geometry::vertex::*;
use glam::{Vec2, Vec3A};
use std::mem;
use std::sync::*;
use super::aabb::*;
use std::cmp::Ordering;
use rand::{Rng};

pub struct Node {
    left: Arc<dyn Traceable>,
    right: Arc<dyn Traceable>,
    aabb: AABB
}

impl Node {
    pub fn new(objects: &Vec<Arc<dyn Traceable>>, min_index: usize, max_index: usize) -> Self {
        let mut objects = objects[min_index..max_index].to_vec(); // Create a modifiable array of the source scene objects
        let axis = rand::thread_rng().gen_range(0..2);

        let count = objects.len();

        let mut left: Arc<dyn Traceable> = Arc::new(AABB::new(Vec3A::ZERO, Vec3A::ZERO));
        let mut right: Arc<dyn Traceable> = Arc::new(AABB::new(Vec3A::ZERO, Vec3A::ZERO));

        if count == 1 {
            left = objects[0].clone();
            right = left.clone();
        } else if count > 0 {
            let mid = count / 2;

            objects.sort_by(|a, b| AABB::cmp(a, b, axis));

            left =  Arc::new(Self::new(&objects, 0, mid));
            right = Arc::new(Self::new(&objects, mid, count));
        }

        let aabb = left.bounding_box().extend(&right.bounding_box());
        return Self {
            left: left,
            right: right,
            aabb: aabb
        };
    }
}

impl Traceable for Node {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        if !self.bounding_box().hit(ray, t_min, t_max).is_some() {
            return None;
        }

        let left_hit_option = self.left.hit(ray, t_min, t_max);
        if left_hit_option.is_some() {
            let left_hit = left_hit_option.unwrap();

            let right_hit_option = self.right.hit(ray, t_min, left_hit.t);
            if right_hit_option.is_some() {
                return right_hit_option;
            }

            return Some(left_hit);
        }

        return self.right.hit(ray, t_min, t_max);
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}