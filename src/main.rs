use pupsy_raytracing_engine::engine::{renderer::Renderer, render_context::RenderContext, 
    scene::Scene};
use std::env;
use std::sync::{Arc};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut scene = Scene::new();

    scene.load_debug();

    for (i, arg) in args.iter().enumerate() {
        if arg == "--in" {
            scene.load_gltf(args[i + 1].as_str());
        }
    }

    // Build bvh
    scene.build_bvh();

    let render_context = Arc::new(RenderContext{scene : scene, spp : 100,
        max_depth : 5});
    let renderer = Renderer{};

    for (index, camera) in render_context.scene.cameras.iter().enumerate() {
        renderer.render(camera.clone(), render_context.clone());
    }
}