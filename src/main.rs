use pupsy_raytracing_engine::engine::{renderer::Renderer, render_context::RenderContext, camera::Camera, 
    scene::Scene, render_target::RenderTarget};
use pupsy_raytracing_engine::engine::math::ray::{*};
use image::{ImageBuffer};
use std::env;
use glam::{Vec3A};

fn main() {
    let args: Vec<String> = env::args().collect();

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 512;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u32;

    let origin = Vec3A::new(0.0, 0.0, 0.0);
    let direction = Vec3A::new(0.0, 0.0, 1.0);

    let camera = Camera{aspect_ratio: ASPECT_RATIO, width: 2.0 * ASPECT_RATIO,
        height: 2.0, focal_length: 1.0, ray: Ray{origin: origin, direction: direction}};

    let render_target = RenderTarget::new(ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32));

    let mut scene = Scene::new();

    scene.load_debug();

    for (i, arg) in args.iter().enumerate() {
        if arg == "--in" {
            scene.load_gltf(args[i + 1].as_str());
        }
    }

    let render_context = RenderContext{render_target : render_target, scene : scene, spp : 100,
        max_depth : 50};

    let renderer = Renderer{};

    renderer.render(camera, render_context);
}