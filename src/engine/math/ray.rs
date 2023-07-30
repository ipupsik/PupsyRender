use glam::{Vec3A};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray {
    pub fn at(self, t :f32) -> Vec3A {
        self.origin + self.direction * t
    }
}