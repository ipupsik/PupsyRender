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
use std::sync::{Arc};

pub struct Mesh {
    pub material : Arc<Box<dyn Material>>,

    geometry : Vec<Arc<dyn Traceable>>,
}

impl Mesh {
    pub fn new(material : Arc<Box<dyn Material>>) -> Self {
        Self{material : material.clone(), geometry : Vec::new()}
    }

    pub fn add_geometry(&mut self, geometry : Arc<dyn Traceable>) {
        self.geometry.push(geometry.clone());
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitResult> {
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

        min_hit_result.material = Arc::downgrade(&self.material);

        Some(min_hit_result)
    }
}

unsafe impl Send for Mesh {}
unsafe impl Sync for Mesh {}