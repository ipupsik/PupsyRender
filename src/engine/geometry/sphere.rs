use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::geometry::traceable::{*};

pub struct Sphere {
    radius : f64,
    position: Vector3,
}

impl Sphere {

}

impl Traceable for Sphere {
    fn hit(&self, ray: Ray) -> Vector3 {
        Vector3{vec: [0.0, 0.0, 0.0]}
    }
}