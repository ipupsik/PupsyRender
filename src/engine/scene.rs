use std::vec;
use crate::engine::mesh::Mesh;
use crate::engine::math::vector3::{*};

use super::geometry::sphere::Sphere;

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self{meshes : Vec::new()}
    }

    pub fn load_debug(&mut self) {
        let mut mesh : Mesh = Mesh::new();
        mesh.add_sphere(Sphere{radius : 0.5, position : Vector3::new(0.0,0.0,-1.0)});

        self.meshes.push(mesh);
    }
}