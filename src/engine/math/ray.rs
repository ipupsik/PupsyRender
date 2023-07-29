use crate::engine::math::vector3::{*};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(self, t :f64) -> Vector3 {
        self.origin + self.direction * t
    }
}