use crate::engine::geometry::traceable::*;
use super::bvh::aabb::*;
use std::sync::*;

pub mod aabb;
pub mod node;

fn box_compare(a: &Arc<dyn Traceable>, b: Arc<dyn Traceable>, axis: usize) -> bool {
    a.bounding_box().min[axis] < b.bounding_box().min[axis]
}