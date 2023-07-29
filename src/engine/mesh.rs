use std::vec::Vec;
use std::option::Option;
use crate::engine::material::{*};
use crate::engine::math::vector3::{*};
use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::geometry::sphere::{*};
use crate::engine::geometry::triangle::{*};

pub struct Mesh {
    material : Material,
    spheres : Vec<Sphere>,
    triangles : Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self{material : Material {  }, 
            spheres : Vec::new(), triangles : Vec::new()}
    }

    pub fn add_sphere(&mut self, sphere : Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_triangle(&mut self, triangle : Triangle) {
        self.triangles.push(triangle);
    }

    pub fn hit(&self, ray: Ray) -> Option<Vector3> {
        let mut success : bool = false;
        let mut min_t : f64 = 0.0;
        let mut min_position : Vector3 = ray.origin;

        for traceable in self.spheres.iter() {
            let hit_option: Option<Vector3>  = traceable.hit(ray);
            if hit_option.is_some() {
                let hit_position: Vector3 = hit_option.unwrap();
                let t : f64 = (hit_position - ray.origin).length();
                success = true;
                if t < min_t {
                    min_t = t;
                    min_position = hit_position;
                }
            }
        }

        if success {
            return None;
        }
        Some(min_position)
    }
}