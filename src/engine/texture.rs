pub mod texture2d;

use std::sync::*;

use image::DynamicImage;

#[derive(Clone)]
pub struct Texture {
    dimensions: Vec<u32>,
    bytes_per_component: u32,
    components_per_pixel: i32,
    //buffer: Arc<Vec<u8>>,
    raw_texture: DynamicImage,
}

impl Texture {
    pub fn null() -> Self {
        Self {
            dimensions: vec![],
            bytes_per_component: 0,
            components_per_pixel: 0,
            //buffer: Arc::new(Vec::new()),
            raw_texture: DynamicImage::new_bgr8(0, 0),
        }
    }

    pub fn new(dimensions: Vec<u32>, bytes_per_component: u32,
        components_per_pixel: i32, raw_texture: DynamicImage) -> Self {
        Self {
            dimensions: dimensions,
            bytes_per_component: bytes_per_component, 
            components_per_pixel: components_per_pixel,
            //buffer: buffer.clone(),
            raw_texture: raw_texture,
        }
    }
}