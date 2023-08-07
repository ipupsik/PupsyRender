use crate::engine::material::Material;
use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::sync::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A,
    pub dummy_material: Arc<dyn Material>
}

impl AABB {
    pub fn new(min: Vec3A, max: Vec3A, dummy_material: Arc<dyn Material>) -> Self {
        Self {
            min: min, 
            max: max,
            dummy_material: dummy_material.clone()
        }
    }

    pub fn extend(&self, other_aabb: &AABB) -> Self {
        AABB::new(
            self.min.min(other_aabb.min),
            self.max.max(other_aabb.max),
            self.dummy_material.clone()
        )
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
}

impl Traceable for AABB {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let aabb_t_min = (self.min - ray.origin) / ray.direction;
        let aabb_t_max = (self.max - ray.origin) / ray.direction;
        let t1 = aabb_t_min.min(aabb_t_max);
        let t2 = aabb_t_min.max(aabb_t_max);
        let t_near = t1.max_element().max(t_min);
        let t_far = t2.min_element().min(t_max);

        if t_near <= t_far && t_far >= 0.0 {
            return Some(HitResult{
                position : Vec3A::ZERO, 
                t : 0.0, 
                normal : Vec3A::ZERO, 
                binormal : Vec3A::ZERO, 
                tangent : Vec3A::ZERO, 
                uvs: Vec::new(), 
                front_face: false, 
                material: self.dummy_material.clone()
            });
        }
        return None;
    }

    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        0.0
    }

    fn random(&self) -> Vec3A {
        Vec3A::ZERO
    }

    fn bounding_box(&self) -> &AABB {
        self
    }
}