use glam::{Vec3A};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray {
    pub const fn new() -> Self {
        Self {
            origin: Vec3A::ZERO,
            direction: Vec3A::ZERO,
        }
    }

    pub fn at(self, t :f32) -> Vec3A {
        self.origin + self.direction * t
    }
}