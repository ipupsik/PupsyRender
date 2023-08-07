use glam::{Vec3A};

pub struct Vertex {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub binormal: Vec3A,
    pub tangent: Vec3A,
    pub uvs: Vec<Vec3A>
}

impl Vertex {
    pub fn new(position : Vec3A, normal: Vec3A, binormal: Vec3A,
        tangent: Vec3A, uvs: Vec<Vec3A>) -> Self {
        Self{ position : position,
        normal : normal,
        binormal: binormal,
        tangent: tangent,
        uvs : uvs}
    }
}