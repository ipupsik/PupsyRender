use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use glam::{Vec3A};

use std::rc::{*};

pub struct Sphere {
    pub radius : f32,
    pub position: Vec3A,
}

impl Sphere {

}

impl Traceable for Sphere {
    fn hit(&self, ray: Ray) -> Option<HitResult> {
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
        if t < 0.0 {
            t = (-half_b + discriminant_sqrt) / a;
            if t < 0.0 {
                return None;
            }
        }
        let position = ray.at(t);
        let mut normal = (position - self.position) / self.radius;
        if normal.dot(ray.direction) > 0.0 {
            normal = normal * -1.0;
        }

        Some(HitResult{position : position, t : t, normal : normal, material : Weak::new()})
    }
}