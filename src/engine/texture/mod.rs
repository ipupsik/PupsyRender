pub mod texture2d;

use glam::{Vec2, Vec3A};

use image::DynamicImage;

#[derive(Clone)]
pub struct Texture {
    dimensions: Vec<u32>,
    bytes_per_component: u32,
    components_per_pixel: i32,
    raw_texture: DynamicImage,
    uv_index: usize,
}

impl Texture {
    pub fn null() -> Self {
        Self {
            dimensions: vec![],
            bytes_per_component: 0,
            components_per_pixel: 0,
            raw_texture: DynamicImage::new_bgr8(0, 0),
            uv_index: 0,
        }
    }

    pub fn new(dimensions: Vec<u32>, bytes_per_component: u32,
        components_per_pixel: i32, raw_texture: DynamicImage) -> Self {
        Self {
            dimensions: dimensions,
            bytes_per_component: bytes_per_component, 
            components_per_pixel: components_per_pixel,
            raw_texture: raw_texture,
            uv_index: 0,
        }
    }

    pub fn set_uv_index(&mut self, uv_index: usize) {
        self.uv_index = uv_index;
    }

    pub fn get_uv_by_index(&self, uvs: &Vec<Vec3A>) -> Vec2{
        for uv in uvs.iter() {
            if uv.z as usize == self.uv_index {
                return Vec2::new(uv.x, uv.y);
            }
        }

        println!("Invalid uv set");
        return Vec2::ZERO;
    }
}