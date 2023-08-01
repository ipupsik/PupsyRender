use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use crate::engine::math::ray::{*};
use glam::{Vec3A};
use crate::engine::scene::{*};
use crate::engine::geometry::traceable::HitResult;
use rand::Rng;

use super::material::Material;
use super::material::diffuse::{*};

use std::thread::{self};
use std::sync::{*};

extern crate num_cpus;

pub struct Renderer {

}

impl Renderer {
    fn gamma_correction(input: Vec3A) -> Vec3A {
        Vec3A::new(
            input.x.powf(1.0 / 2.2),
            input.y.powf(1.0 / 2.2),
            input.z.powf(1.0 / 2.2))
    }

    fn tone_mapping(input: Vec3A) -> Vec3A {
       input / (Vec3A::ONE + input)
    }

    fn sample_scene(ray : &Ray, scene : &Scene, depth : u64) -> Vec3A {
        let mut success = false;
        let mut min_hit_result = HitResult::new();
        let mut hit_material:Arc<dyn Material> = Arc::new(DiffuseMaterial{});

        for mesh in scene.meshes.iter() {
            let hit_option: Option<HitResult>  = mesh.hit(ray);

            if hit_option.is_some() {
                let hit_result = hit_option.unwrap();
                if hit_result.t < min_hit_result.t {
                    success = true;
                    min_hit_result = hit_result;
                    hit_material = mesh.material.clone();
                }
            }
        }

        if !success {
            let t = 0.5 * (ray.direction.y + 1.0);
            return (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0);
        }

        let sample = hit_material.sample(&min_hit_result);

        if depth > 1 {
            let scatter_option = hit_material.scatter(ray, &min_hit_result);

            if scatter_option.is_some() {
                let scatter_vector = scatter_option.unwrap();

                let new_ray = Ray{origin : min_hit_result.position, direction : scatter_vector};
                if depth > 1 {
                    return 0.8 * Self::sample_scene(&new_ray, scene, depth - 1) * sample;
                }
            }
        }

        Vec3A::ZERO
    }

    pub fn render(&self, camera : &'static Camera, render_context : &'static RenderContext, output_frame_buffer: &mut Vec<Vec<Vec3A>>) {
        let frame_buffer_len = (render_context.render_target.width * render_context.render_target.height) as usize;

        let num_cores = num_cpus::get();
        let chunk_size = frame_buffer_len / num_cores;

        let mut threads = Vec::with_capacity(num_cores);

        let (tx, rx) =  mpsc::channel();

        let camera = Arc::new(camera.clone());
        let render_context = Arc::new(render_context.clone());
        for thr in 0..num_cores
        {
            let index_begin = thr * chunk_size;
            let mut index_end = (thr + 1) * chunk_size;
            if thr == num_cores - 1
            {
                index_end = frame_buffer_len;
            }

            output_frame_buffer.push(Vec::new());
            output_frame_buffer[thr].reserve((index_end - index_begin) as usize);

            let camera = Arc::clone(&camera);
            let render_context = Arc::clone(&render_context);

            let tx =  tx.clone();
            threads.push(
                thread::spawn(move || {
                    let mut frame_buffer_slice = vec![Vec3A::ZERO; index_end - index_begin];
                    for i in 0..index_end - index_begin
                    {
                        frame_buffer_slice[i] = Vec3A::ONE * 255.99;
                        let x = (index_begin + i) as u32 % render_context.render_target.width;
                        let y = (index_begin + i) as u32 / render_context.render_target.width;

                        let mut scene_color = Vec3A::new(0.0, 0.0, 0.0);
                        for _ in 0..render_context.spp {
                            let u = (x as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.width - 1) as f32;
                            let v = (y as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.height - 1) as f32;
            
                            let ray = camera.get_ray(u, 1.0 - v);
            
                            let mut current_sample = Self::sample_scene(&ray, 
                                &render_context.scene, render_context.max_depth);
         
                            scene_color = scene_color + current_sample / render_context.spp as f32;
                        }

                        //scene_color = Self::tone_mapping(scene_color);
                        scene_color = Self::gamma_correction(scene_color);  
                        scene_color = scene_color * 255.999;

                        frame_buffer_slice[i] = scene_color;
                    }
                    tx.send((frame_buffer_slice, thr)).unwrap();
                })
            );
        }

        for thr in threads
        {
            thr.join().unwrap();
        }

        for v in rx.iter().take(num_cores){
            let (frame_buffer_slice, thread_index) = v;
            output_frame_buffer[thread_index].extend(frame_buffer_slice);
        }
    }
}