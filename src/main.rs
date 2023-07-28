use pupsy_raytracing_engine::engine::{vector3::Vector3, renderer::Renderer, render_context::RenderContext, ray::Ray, camera::Camera, render_target::RenderTarget};
use image::{RgbImage, ImageBuffer, Rgb};

fn main() {

    const IMAGE_WIDTH: u64 = 512;
    const IMAGE_HEIGHT: u64 = 512;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

    let origin = Vector3{vec:[0.0, 0.0, 0.0]};
    let direction = Vector3{vec:[0.0, 0.0, 1.0]};

    let camera = Camera{aspect_ratio: ASPECT_RATIO, width: 2.0f64 * ASPECT_RATIO,
        height: 2.0, focal_length: 1.0f64, ray: Ray{origin: origin, direction: direction}};

    let render_target = RenderTarget::new(ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32));

    let render_context = RenderContext{render_target : render_target};

    let renderer = Renderer{};

    renderer.render(camera, render_context);
}