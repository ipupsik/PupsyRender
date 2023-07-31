use std::vec::Vec;
use std::option::Option;
use crate::engine::material::{*};
use crate::engine::material::diffuse::{*};
use glam::{Vec3A};
use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::geometry::sphere::{*};
use crate::engine::geometry::triangle::{*};

use std::rc::{*};

use super::geometry::traceable;

pub struct Mesh {
    pub material : Rc<Material>,

    geometry : Vec<Rc<dyn Traceable>>,
}

impl Mesh {
    pub fn new(material : Rc<Material>) -> Self {
        Self{material : material, geometry : Vec::new()}
    }

    pub fn add_geometry(&mut self, geometry : Rc<dyn Traceable>) {
        self.geometry.push(geometry.clone());
    }

    pub fn hit(&self, ray: Ray) -> Option<HitResult> {
        let mut success = false;
        let mut min_hit_result = HitResult::new();

        for traceable in self.geometry.iter() {
            let hit_option: Option<HitResult> = traceable.hit(ray, 0.001, f32::MAX);
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

        min_hit_result.material = Rc::downgrade(&self.material);

        Some(min_hit_result)
    }
}