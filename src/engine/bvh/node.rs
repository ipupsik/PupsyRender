use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::geometry::vertex::*;
use glam::{Vec2, Vec3A};
use std::mem;
use std::sync::*;
use super::aabb::*;
use rand::{Rng};
use crate::engine::bvh::*;

pub struct Node {
    left: Arc<dyn Traceable>,
    right: Arc<dyn Traceable>,
    aabb: AABB
}

impl Node {
    pub fn new(objects: &Vec<Arc<dyn Traceable>>, min_index: usize, max_index: usize) -> Self {
        let objects = objects.clone(); // Create a modifiable array of the source scene objects

        let rand_axis = rand::thread_rng().gen_range(0..2);

        // Sort by axis

        let count = max_index - min_index;
        let left;
        let right;

        if count == 1 {
            left = objects[min_index].clone();
            right = left.clone();
        } else if count == 2 {
            left = objects[min_index].clone();
            right = objects[max_index].clone();
        } else {
            let mid = min_index + count / 2;

            left =  Arc::new(Self::new(&objects, min_index, mid));
            right = Arc::new(Self::new(&objects, mid, max_index));
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
        if !self.aabb.hit(ray, t_min, t_max).is_some() {
            return None;
        }

        let left_hit_option = self.left.hit(ray, t_min, t_max);

        let right_hit_option = self.right.hit(ray, t_min, t_max);
        if right_hit_option.is_some() {
            if right_hit_option.is_some() {
                let right_hit = right_hit_option.unwrap();
                let left_hit = left_hit_option.unwrap();
                if left_hit.t < right_hit.t {return Some(left_hit)} else {return Some(right_hit)};
            }
            return right_hit_option;
        }

        return left_hit_option;
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}