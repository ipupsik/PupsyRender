use glam::{Vec3A, Vec2};

pub struct Vertex {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub uv: Vec2
}

impl Vertex {
    pub const fn new(position : Vec3A) -> Self {
        Self{ position : position,
        normal : Vec3A::ZERO,
        uv : Vec2::ZERO}
    }
}