use crate::engine::{math::ray::*, material::Material};
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use super::aabb::*;
use rand::{Rng};

pub struct Node {
    left: Arc<dyn Mesh>,
    right: Arc<dyn Mesh>,
    aabb: AABB
}

impl Node {
    pub fn new(objects: &Vec<Arc<dyn Mesh>>, min_index: usize, max_index: usize, dummy_material: &Arc<dyn Material>) -> Self {
        let mut objects = objects[min_index..max_index].to_vec(); // Create a modifiable array of the source scene objects
        let axis = rand::thread_rng().gen_range(0..2);

        let count = objects.len();

        let mut left: Arc<dyn Mesh> = Arc::new(AABB::new(Vec3A::ZERO, Vec3A::ZERO, dummy_material.clone()));
        let mut right: Arc<dyn Mesh> = Arc::new(AABB::new(Vec3A::ZERO, Vec3A::ZERO, dummy_material.clone()));

        if count == 1 {
            left = objects[0].clone();
            right = left.clone();
        } else if count > 0 {
            let mid = count / 2;

            objects.sort_by(|a, b| AABB::cmp(a, b, axis));

            left =  Arc::new(Self::new(&objects, 0, mid, dummy_material));
            right = Arc::new(Self::new(&objects, mid, count, dummy_material));
        }

        let aabb = left.bounding_box().extend(&right.bounding_box());
        return Self {
            left: left,
            right: right,
            aabb: aabb
        };
    }
}

impl Mesh for Node {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (Option<HitResult>, &dyn Mesh) {
        let (main_hit_result, _) = self.bounding_box().hit(ray, t_min, t_max);
        if !main_hit_result.is_some() {
            return (None, self);
        }

        let (left_hit_option, left_traceable) = self.left.hit(ray, t_min, t_max);
        if left_hit_option.is_some() {
            let left_hit = left_hit_option.unwrap();

            let (right_hit_option, right_traceable) = self.right.hit(ray, t_min, left_hit.t);
            if right_hit_option.is_some() {
                return (right_hit_option, right_traceable);
            }

            return (Some(left_hit), left_traceable);
        }

        return self.right.hit(ray, t_min, t_max);
    }

    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        0.0
    }

    fn random(&self) -> Vec3A {
        Vec3A::ZERO
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }

    fn material(&self) -> &Arc<dyn Material> {
        return &self.aabb.dummy_material;
    }
}