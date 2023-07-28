use crate::engine::vector3::{*};

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(self, t :f64) -> Vector3 {
        self.origin + self.direction * t
    }
}