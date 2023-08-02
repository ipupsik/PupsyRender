use pupsy_raytracing_engine::engine::{renderer::Renderer, render_context::RenderContext, camera::Camera, 
    scene::Scene, render_target::RenderTarget};
use pupsy_raytracing_engine::engine::math::ray::*;
use image::{ImageBuffer};
use std::env;
use std::sync::{Arc};
use glam::{Vec3A};
use image::{Rgb};

fn main() {
    let args: Vec<String> = env::args().collect();

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 512;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f32 = 3.8;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;

    let origin = Vec3A::new(0.0, 0.0, -2.0);
    let direction = Vec3A::new(0.0, 0.0, 1.0);

    let camera = Arc::new(Camera{aspect_ratio: ASPECT_RATIO, width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT, focal_length: 1.0, ray: Ray{origin: origin, direction: direction}});


    let mut rgb_frame_buffer = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let render_target = RenderTarget{
        width: IMAGE_WIDTH as u32,
        height: IMAGE_HEIGHT as u32,
    };

    let mut scene = Scene::new();

    scene.load_debug();

    for (i, arg) in args.iter().enumerate() {
        if arg == "--in" {
            scene.load_gltf(args[i + 1].as_str());
        }
    }

    // Build bvh
    scene.build_bvh();

    let render_context = Arc::new(RenderContext{render_target : render_target, scene : scene, spp : 10,
        max_depth : 5});

    let renderer = Renderer{};

    let mut frame_buffer = Vec::new();
    renderer.render(camera.clone(), render_context.clone(), &mut frame_buffer);

    for (x, y, pixel) in rgb_frame_buffer.enumerate_pixels_mut() {
        let mut linear_index = (y * render_context.render_target.width + x) as usize;
        for chunk in frame_buffer.iter() {
            if linear_index < chunk.len() {
                let scene_color = chunk[linear_index];

                *pixel = Rgb([scene_color.x as u8, scene_color.y as u8, scene_color.z as u8]);
                break;
            }
            linear_index -= chunk.len();
        }
    }

    rgb_frame_buffer.save("image.png").unwrap();
}