use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::geometry::vertex::{*};
use glam::{Vec3A};

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub normal: Vec3A,
}

impl Triangle {
    pub fn new(v1 : Vertex, v2 : Vertex, v3 : Vertex) -> Self{
        Self {
            vertices : [v1, v2, v3],
            normal: Vec3A::ZERO
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: Ray) -> Option<HitResult> {
        None
    }
}