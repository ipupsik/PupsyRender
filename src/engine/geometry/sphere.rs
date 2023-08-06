use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use super::bvh::aabb::*;
use crate::engine::material::*;

use std::{sync::*};

pub struct Sphere {
    pub material: Arc<dyn Material>,

    pub radius : f32,
    pub position: Vec3A,
}

impl Sphere {

}

impl Traceable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let oc: Vec3A = ray.origin - self.position;
        let a: f32 = ray.direction.dot(ray.direction);
        let half_b: f32 = oc.dot(ray.direction);
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut t = (-half_b - discriminant_sqrt) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant_sqrt) / a;
            if t < t_min {
                return None;
            }
        }
        let position = ray.at(t);
        let mut front_face = true;
        let mut normal = (position - self.position) / self.radius;
        if normal.dot(ray.direction) > 0.0 {
            normal = normal * -1.0;
            front_face = false;
        }

        Some(HitResult{position : position, t : t, normal : normal, 
            uvs: Vec::new(), front_face: front_face, material: self.material.clone()})
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.position - Vec3A::new(self.radius, self.radius, self.radius),
            self.position + Vec3A::new(self.radius, self.radius, self.radius)
        )
    }
}