pub mod texture2d;

use std::sync::*;

#[derive(Clone)]
pub struct Texture {
    dimensions: Vec<u32>,
    bytes_per_component: u32,
    components_per_pixel: i32,
    buffer: Arc<Vec<u8>>
}

impl Texture {
    pub fn null() -> Self {
        Self {
            dimensions: vec![],
            bytes_per_component: 0,
            components_per_pixel: 0,
            buffer: Arc::new(Vec::new()),
        }
    }

    pub fn new(dimensions: Vec<u32>, bytes_per_component: u32,
        components_per_pixel: i32, buffer: Arc<Vec<u8>>) -> Self {
        Self {
            dimensions: dimensions,
            bytes_per_component: bytes_per_component, 
            components_per_pixel: components_per_pixel,
            buffer: buffer.clone(),
        }
    }
}