use glam::{Vec3A, Vec2};
use std::{collections::HashMap, hash};

pub struct Vertex {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub uvs: Vec<Vec3A>
}

impl Vertex {
    pub fn new(position : Vec3A, normal: Vec3A, uvs: Vec<Vec3A>) -> Self {
        Self{ position : position,
        normal : normal,
        uvs : uvs}
    }
}