use crate::engine::math::ray::{*};

pub struct Camera {
    pub aspect_ratio: f64,
    pub width: f64,
    pub height: f64,
    pub focal_length: f64,
    pub ray: Ray
}