use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use image::{RgbImage, ImageBuffer, Rgb};

pub struct Renderer {

}

impl Renderer {
    pub fn render(&self, camera : Camera, mut render_context : RenderContext) {
        for (x, y, pixel) in render_context.render_target.frame_buffer.enumerate_pixels_mut() {
            let r = x as f64 / (render_context.render_target.width - 1) as f64;
            let g = y as f64 / (render_context.render_target.height - 1) as f64;
            let b = 0.25;
    
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
    
            *pixel = Rgb([ir, ig, ib]);
        }
    
        render_context.render_target.frame_buffer.save("content/image.png").unwrap();
    }
}