use crate::engine::math::ray::{*};
use crate::engine::geometry::traceable::{*};
use glam::{Vec3A};

pub struct Triangle {
    positions: [Vec3A; 3],
}

impl Triangle {

}

impl Traceable for Triangle {
    fn hit(&self, ray: Ray) -> Option<HitResult> {
        None
    }
}