use crate::engine::math::vector3::{*};
use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use crate::engine::math::vector3::{*};

pub struct Triangle {
    positions: [Vector3; 3],
}

impl Triangle {

}

impl Traceable for Triangle {
    fn hit(&self, ray: Ray) -> Option<HitResult> {
        None
    }
}