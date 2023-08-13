use std::option::Option;

use crate::engine::{math::ray::*, material::diffuse::DiffuseMaterial};
use crate::engine::material::*;
use std::sync::*;
use super::bvh::aabb::*;
use glam::{Vec3A};

pub trait Traceable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (Option<HitResult>, &dyn Traceable);
    fn pdf(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32;
    fn random(&self) -> Vec3A;
    fn bounding_box(&self) -> &AABB;
    fn centroid(&self) -> &Vec3A;

    fn material(&self) -> &Arc<dyn Material>;
}

#[derive(Clone)]
pub struct HitResult {
    pub position: Vec3A,
    pub t: f32,
    pub normal: Vec3A,
    pub binormal: Vec3A,
    pub tangent: Vec3A,
    pub uvs: Vec<Vec3A>,
    pub front_face: bool,
}

impl HitResult {

}