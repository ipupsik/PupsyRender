use image::{RgbImage};

pub struct RenderTarget {
    pub width: u32,
    pub height: u32
}

impl RenderTarget {
    pub const fn new() -> Self {
        let width = 0;
        let height = 0;
        Self{width: width, height: height}
    }
}