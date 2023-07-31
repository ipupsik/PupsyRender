pub mod diffuse;
pub mod metal;
pub mod normal;

use crate::engine::math::ray::{*};
use glam::{Vec3A};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};
use crate::engine::geometry::traceable::{*};

use std::rc::{*};

pub struct Material {
    pub scatter : Rc<dyn Scatter>,
    pub sample : Rc<dyn Sample>,
}

impl Material {

}

pub trait Scatter {
    fn scatter(&self, ray: Ray, hit_result : &HitResult) -> Option<Vec3A>;
}

pub trait Sample {
    fn sample(&self, hit_result : &HitResult) -> Vec3A;
}