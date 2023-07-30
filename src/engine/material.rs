pub mod diffuse;
pub mod metal;

use crate::engine::math::ray::{*};
use glam::{Vec3A};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};
use crate::engine::geometry::traceable::{*};

use std::rc::{*};

pub struct Material {
    pub scatter : Rc<dyn Scatter>,
}

impl Material {

}

pub trait Scatter {
    fn scatter(&self, ray: Ray, hit_result : &HitResult) -> Vec3A;
}