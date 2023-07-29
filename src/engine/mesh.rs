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

    pub fn hit(&self, ray: Ray) -> Option<HitResult> {
        let mut success = false;
        let mut min_hit_result = HitResult::new();

        for traceable in self.spheres.iter() {
            let hit_option: Option<HitResult> = traceable.hit(ray);
            if hit_option.is_some() {
                let hit_result = hit_option.unwrap();
                if hit_result.t < min_hit_result.t {
                    success = true;
                    min_hit_result = hit_result;
                }
            }
        }

        if !success {
            return None;
        }
        Some(min_hit_result)
    }
}