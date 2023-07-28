use image::{RgbImage};

pub struct RenderTarget {
    pub frame_buffer: RgbImage,
    pub width: u32,
    pub height: u32
}

impl RenderTarget {
    pub fn new(frame_buffer: RgbImage) -> Self {
        let width = frame_buffer.width();
        let height = frame_buffer.height();
        Self{frame_buffer: frame_buffer, width: width, height: height}
    }
}