use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use super::bvh::aabb::*;
use crate::engine::material::*;
use crate::engine::math::utils::*;

use std::{sync::*};

pub struct Sphere {
    pub material: Arc<dyn Material>,

    pub aabb: AABB,
    pub radius : f32,
    pub position: Vec3A,
}

impl Sphere {
    pub fn new( material: Arc<dyn Material>,
        radius : f32, position: Vec3A) -> Self {
        let aabb = AABB::new(
            position - Vec3A::new(radius, radius, radius),
            position + Vec3A::new(radius, radius, radius),
        );

        Self {
            material: material,
            radius: radius,
            position: position,
            aabb: aabb,
        }
    }
}

impl Traceable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (Option<HitResult>, &dyn Traceable) {
        let oc: Vec3A = ray.origin - self.position;
        let a: f32 = ray.direction.dot(ray.direction);
        let half_b: f32 = oc.dot(ray.direction);
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (None, self);
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut t = (-half_b - discriminant_sqrt) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant_sqrt) / a;
            if t < t_min {
                return (None, self);
            }
        }
        let position = ray.at(t);
        let mut front_face = true;
        let mut normal = (position - self.position) / self.radius;
        if normal.dot(ray.direction) > 0.0 {
            normal = normal * -1.0;
            front_face = false;
        }

        (Some(HitResult{
            position : position, 
            t : t, 
            normal : normal, 
            binormal : normal, 
            tangent : normal, 
            uvs: Vec::new(), 
            front_face: front_face,
        }), self)
    }

    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        let (hit_result_option, traceable) = self.hit(ray, t_min, t_max);
        if (!hit_result_option.is_some()) {
            return 0.0;
        }

        let cos_theta_max = (1.0 - self.radius * self.radius / (self.position - ray.origin).length_squared()).sqrt();
        let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

        return 1.0 / solid_angle;
    }

    fn random(&self) -> Vec3A {
        self.position + random_in_unit_sphere() * self.radius
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }

    fn centroid(&self) -> &Vec3A {
        &self.position
    }

    fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }
}