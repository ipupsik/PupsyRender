use std::vec;
use crate::engine::mesh::Mesh;
use glam::{Vec3A};
use crate::engine::material::{*};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};

use super::geometry::sphere::Sphere;

use std::rc::{*};
use std::io;
use std::fs;

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            meshes : Vec::new(),
        }
    }

    fn print_tree(node: &gltf::Node, depth: i32) {
        for _ in 0..(depth - 1) {
            print!("  ");
        }
        print!(" -");
        print!(" Node {}", node.index());
        #[cfg(feature = "names")]
        print!(" ({})", node.name().unwrap_or("<Unnamed>"));
        println!();
    
        for child in node.children() {
            Scene::print_tree(&child, depth + 1);
        }
    }

    pub fn load_gltf(&mut self, path: &str) {
        let file = fs::File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();
        for scene in gltf.scenes() {
            print!("Scene {}", scene.index());
            #[cfg(feature = "names")]
            print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
            println!();
            for node in scene.nodes() {
                Scene::print_tree(&node, 1);
            }
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material_data = DiffuseMaterial{};
        let diffuse_material = Material{scatter : Rc::new(diffuse_material_data)};

        let mut mesh : Mesh = Mesh::new(Rc::new(diffuse_material));
        mesh.add_sphere(Sphere{radius : 0.5, position : Vec3A::new(0.0, 0.0, 1.2)});
        mesh.add_sphere(Sphere{radius : 100.0, position : Vec3A::new(0.0, -100.5, 1.0)});
        self.meshes.push(mesh);

        let metal_material_data = MetalMaterial{metalness : 0.3};
        let metal_material = Material{scatter : Rc::new(metal_material_data)};
        
        let mut mesh : Mesh = Mesh::new(Rc::new(metal_material));
        mesh.add_sphere(Sphere{radius : 0.5, position : Vec3A::new(1.0, 0.0, 1.2)});
        mesh.add_sphere(Sphere{radius : 0.5, position : Vec3A::new(-1.0, 0.0, 1.2)});
        self.meshes.push(mesh);
    }
}