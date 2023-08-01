pub mod texture2d;

use std::sync::*;

#[derive(Clone)]
pub struct Texture {
    width: u32,
    height: u32,
    buffer: Arc<Vec<u8>>
}

impl Texture {
    pub fn new(width: u32, height: u32, buffer: Arc<Vec<u8>>) -> Self {
        Self {
            width: width,
            height: height,
            buffer: buffer.clone(),
        }
    }
}