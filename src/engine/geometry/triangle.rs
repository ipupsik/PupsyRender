use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::geometry::vertex::*;
use glam::{Vec2, Vec3A};
use std::collections::HashMap;
use super::bvh::aabb::*;
use crate::engine::material::*;
use std::sync::*;

pub struct Triangle {
    pub material: Arc<dyn Material>,

    pub vertices: [Vertex; 3],
    pub normal: Vec3A,
}

impl Triangle {
    pub const EPSILON: f32 = 1e-8;
    
    pub fn new(material: Arc<dyn Material>, v1 : Vertex, v2 : Vertex, v3 : Vertex) -> Self{
        Self {
            material: material.clone(),
            vertices : [v1, v2, v3],
            normal: Vec3A::ZERO
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let v0v1 = self.vertices[1].position - self.vertices[0].position;
        let v0v2 = self.vertices[2].position - self.vertices[0].position;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        // if the determinant is negative, the triangle is 'back facing'
        // if the determinant is close to 0, the ray misses the triangle
        if det.abs() < Triangle::EPSILON {
            return None;
        }

        let front_face = det < Triangle::EPSILON;

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertices[0].position;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = v0v2.dot(qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let mut uvs = self.vertices[0].uvs.clone();
        for uv in uvs.iter_mut() {
            *uv = self.vertices[0].uvs[uv.z as usize] * (1.0 - v - u) + self.vertices[1].uvs[uv.z as usize] * u + self.vertices[2].uvs[uv.z as usize] * v;
        }
        
        let normal = self.vertices[0].normal * (1.0 - v - u) + self.vertices[1].normal * u + self.vertices[2].normal * v;

        return Some(HitResult { position: ray.at(t), t: t, normal: normal, 
            uvs: uvs, front_face: front_face, material: self.material.clone() });
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.vertices[0].position.min(self.vertices[1].position.min(self.vertices[2].position)),
            self.vertices[0].position.max(self.vertices[1].position.max(self.vertices[2].position)),
        )
    }
}