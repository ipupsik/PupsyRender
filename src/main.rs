use pupsy_render::engine::{renderer::Renderer, render_context::RenderContext, 
    scene::Scene};
use std::env;
use std::process::exit;
use std::sync::{Arc};
use pupsy_render::engine::profile::*;

fn main() {
    let mut render_context = RenderContext::new();

    let total_time = Profile::new(format!("Total Time").as_str(), ProfileType::INSTANT);

    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--debug" {
            render_context.scene.load_debug();
        }

        if arg == "--debug_steps" {
            render_context.debug_steps = true;
        }

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

        if arg == "--out" {
            if args.len() > i + 1 {
                let output = args[i + 1].as_str();
                render_context.output = String::from(output);
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

        if arg == "--bounces" {
            if args.len() > i + 1 {
                let depth: u32 = args[i + 1].parse::<u32>().expect("Invalid depth value");
                render_context.max_depth = depth;
            }
            else {
                println!("Empty depth value");
                exit(-1);
            }
        }

        if arg == "--height" {
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

    drop(total_time);
}