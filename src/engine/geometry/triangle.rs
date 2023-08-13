use crate::engine::math::ray::*;
use crate::engine::geometry::traceable::*;
use crate::engine::geometry::vertex::*;
use glam::{Vec3A};
use rand::{Rng};
use super::bvh::aabb::*;
use crate::engine::material::*;
use std::sync::*;

pub struct Triangle {
    pub material: Arc<dyn Material>,
    pub aabb: AABB,

    pub vertices: [Vertex; 3],
    pub centroid: Vec3A,
}

impl Triangle {
    pub const EPSILON: f32 = 1e-8;
    
    pub fn new(material: Arc<dyn Material>, 
        v1 : Vertex, v2 : Vertex, v3 : Vertex) -> Self {

        let aabb = AABB::new(
            v1.position.min(v2.position.min(v3.position)),
            v1.position.max(v2.position.max(v3.position)),
        );

        let centroid = (v1.position + v2.position + v3.position) / 3.0;

        Self {
            material: material.clone(),
            vertices : [v1, v2, v3],
            aabb: aabb,
            centroid: centroid,
        }
    }
}

impl Traceable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (Option<HitResult>, &dyn Traceable) {
        let v0v1 = self.vertices[1].position - self.vertices[0].position;
        let v0v2 = self.vertices[2].position - self.vertices[0].position;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        // if the determinant is negative, the triangle is 'back facing'
        // if the determinant is close to 0, the ray misses the triangle
        if det.abs() < Triangle::EPSILON {
            return (None, self);
        }

        let front_face = det < Triangle::EPSILON;

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertices[0].position;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return (None, self);
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return (None, self);
        }
        let t = v0v2.dot(qvec) * inv_det;

        if t < t_min || t > t_max {
            return (None, self);
        }

        let mut uvs = self.vertices[0].uvs.clone();
        for uv in uvs.iter_mut() {
            *uv = self.vertices[0].uvs[uv.z as usize] * (1.0 - v - u) + self.vertices[1].uvs[uv.z as usize] * u + self.vertices[2].uvs[uv.z as usize] * v;
        }
        
        let normal = self.vertices[0].normal * (1.0 - v - u) + self.vertices[1].normal * u + self.vertices[2].normal * v;
        let binormal = self.vertices[0].binormal * (1.0 - v - u) + self.vertices[1].binormal * u + self.vertices[2].binormal * v;
        let tangent = self.vertices[0].tangent * (1.0 - v - u) + self.vertices[1].tangent * u + self.vertices[2].tangent * v;

        return (Some(HitResult { 
            position: ray.at(t), 
            t: t, 
            normal: normal.normalize(), 
            binormal: binormal.normalize(), 
            tangent: tangent.normalize(), 
            uvs: uvs, 
            front_face: front_face,
        }), self);
    }

    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        let (hit_result_option, _) = self.hit(ray, t_min, t_max);
        if !hit_result_option.is_some() {
            return 0.0;
        }
        let hit_result = hit_result_option.unwrap();

        let area = 0.5 * (self.vertices[0].position - self.vertices[1].position).cross(
            self.vertices[0].position - self.vertices[2].position
        ).length();
        let distance_squared = hit_result.t * hit_result.t;
        let cosine = ray.direction.dot(hit_result.normal).abs();

        return distance_squared / (cosine * area);
    }

    fn random(&self) -> Vec3A {
        let u: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let v: f32 = rand::thread_rng().gen_range(0.0..1.0);

        self.vertices[0].position * (1.0 - v - u) + self.vertices[1].position * u + self.vertices[2].position * v
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }

    fn centroid(&self) -> &Vec3A {
        &self.centroid
    }

    fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }
}