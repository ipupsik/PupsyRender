use std::vec;
use crate::engine::mesh::Mesh;
use crate::engine::math::vector3::{*};
use crate::engine::material::{*};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};

use super::geometry::sphere::Sphere;

use std::rc::{*};

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            meshes : Vec::new(),
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material_data = DiffuseMaterial{};
        let diffuse_material = Material{scatter : Rc::new(diffuse_material_data)};

        let mut mesh : Mesh = Mesh::new(Rc::new(diffuse_material));
        mesh.add_sphere(Sphere{radius : 0.5, position : Vector3::new(0.0, 0.0, 1.2)});
        mesh.add_sphere(Sphere{radius : 100.0, position : Vector3::new(0.0, -100.5, 1.0)});
        self.meshes.push(mesh);

        let metal_material_data = MetalMaterial{metalness : 0.3};
        let metal_material = Material{scatter : Rc::new(metal_material_data)};
        
        let mut mesh : Mesh = Mesh::new(Rc::new(metal_material));
        mesh.add_sphere(Sphere{radius : 0.5, position : Vector3::new(1.0, 0.0, 1.2)});
        mesh.add_sphere(Sphere{radius : 0.5, position : Vector3::new(-1.0, 0.0, 1.2)});
        self.meshes.push(mesh);
    }
}