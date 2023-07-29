use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::geometry::traceable::{*};

pub struct Sphere {
    pub radius : f64,
    pub position: Vector3,
}

impl Sphere {

}

impl Traceable for Sphere {
    fn hit(&self, ray: Ray) -> Option<Vector3> {
        let oc: Vector3 = ray.origin - self.position;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = 2.0 * oc.dot(ray.direction);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        Some(ray.at((-b - discriminant.sqrt()) / (2.0 * a)))
    }
}