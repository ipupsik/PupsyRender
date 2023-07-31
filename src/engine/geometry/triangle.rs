use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::geometry::vertex::{*};
use glam::{Vec2, Vec3A};
use std::rc::{*};

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub normal: Vec3A,
}

impl Triangle {
    pub const Epsilon: f32 = 1e-8;
    
    pub fn new(v1 : Vertex, v2 : Vertex, v3 : Vertex) -> Self{
        Self {
            vertices : [v1, v2, v3],
            normal: Vec3A::ZERO
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: Ray) -> Option<HitResult> {
        let v0v1 = self.vertices[1].position - self.vertices[0].position;
        let v0v2 = self.vertices[2].position - self.vertices[0].position;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        // if the determinant is negative, the triangle is 'back facing'
        // if the determinant is close to 0, the ray misses the triangle
        if det < Triangle::Epsilon {
            return None;
        }
        let invDet = 1.0 / det;

        let tvec = ray.origin - self.vertices[0].position;
        let u = tvec.dot(pvec) * invDet;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * invDet;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = v0v2.dot(qvec) * invDet;
        
        return Some(HitResult { position: ray.at(t), t: t, normal: self.normal, material: Weak::new(), uv: Vec2::new(u, v) });
    }
}