use std::option::Option;

use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};

pub trait Traceable {
    fn hit(&self, ray: Ray) -> Option<Vector3>;
}