use pupsy_raytracing_engine::engine::{renderer::Renderer, render_context::RenderContext, 
    scene::Scene};
use std::env;
use std::process::exit;
use std::sync::{Arc};

fn main() {
    let mut render_context = RenderContext::new();

    render_context.scene.load_debug();

    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--in" {
            if args.len() > i + 1 {
                let input_gltf_file = args[i + 1].as_str();
                render_context.scene.load_gltf(input_gltf_file);
            }
            else {
                println!("Empty input GLTF file");
                exit(-1);
            }
        }

        if arg == "--spp" {
            if args.len() > i + 1 {
                let spp = args[i + 1].parse::<u32>().expect("Invalid spp value");
                render_context.spp = spp;
            }
            else {
                println!("Empty spp value");
                exit(-1);
            }
        }

        if arg == "--depth" {
            if args.len() > i + 1 {
                let depth: u32 = args[i + 1].parse::<u32>().expect("Invalid depth value");
                render_context.max_depth = depth;
            }
            else {
                println!("Empty depth value");
                exit(-1);
            }
        }

        if arg == "--res" {
            if args.len() > i + 1 {
                let resolution: u32 = args[i + 1].parse::<u32>().expect("Invalid depth value");
                render_context.resolution = resolution;
            }
            else {
                println!("Empty depth value");
                exit(-1);
            }
        }
    }

    // Build bvh
    render_context.scene.build_bvh();

    let renderer = Renderer{};

    let render_context = Arc::new(render_context);

    for (index, camera) in render_context.scene.cameras.iter().enumerate() {
        renderer.render(camera.clone(),  render_context.clone());
    }
}