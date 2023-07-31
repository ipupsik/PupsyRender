use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use crate::engine::math::ray::{*};
use glam::{Vec3A};
use crate::engine::scene::{*};
use crate::engine::geometry::traceable::HitResult;
use image::{Rgb};
use rand::Rng;

use super::material::Material;
use super::material::diffuse::{*};

use std::rc::{*};

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

    fn sample_scene(ray : Ray, scene : &Scene, depth : u64) -> Vec3A {
        let mut success = false;
        let mut min_hit_result = HitResult::new();

        for mesh in scene.meshes.iter() {
            let hit_option: Option<HitResult>  = mesh.hit(ray);

            if hit_option.is_some() {
                let hit_result = hit_option.unwrap();
                if hit_result.t < min_hit_result.t {
                    success = true;
                    min_hit_result = hit_result;
                }
            }
        }

        if !success {
            let t = 0.5 * (ray.direction.y + 1.0);
            return (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0);
        }

        let sample = unsafe {(*min_hit_result.material.as_ptr()).sample.sample(&min_hit_result)};

        if depth > 1 {
            let scatter_option = unsafe {(*min_hit_result.material.as_ptr()).scatter.scatter(ray, &min_hit_result)};

            if scatter_option.is_some() {
                let scatter_vector = scatter_option.unwrap();

                let new_ray = Ray{origin : min_hit_result.position, direction : scatter_vector};
                if depth > 1 {
                    return 0.8 * Self::sample_scene(new_ray, scene, depth - 1) * sample;
                }
            }
        }

        Vec3A::ZERO
    }

    pub fn render(&self, camera : Camera, render_context : RenderContext) {
        let mut frame_buffer = render_context.render_target.frame_buffer;
        for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
            let mut scene_color = Vec3A::new(0.0, 0.0, 0.0);
            for _ in 0..render_context.spp {
                let u = (x as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.width - 1) as f32;
                let v = (y as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.height - 1) as f32;

                let ray = camera.get_ray(u, 1.0 - v);

                let mut current_sample = Self::sample_scene(ray, 
                    &render_context.scene, render_context.max_depth);

                //current_sample = Self::tone_mapping(current_sample);
                current_sample = Self::gamma_correction(current_sample);    

                current_sample = current_sample * 255.999;

                scene_color = scene_color + current_sample / render_context.spp as f32;
            }
    
            *pixel = Rgb([scene_color.x as u8, scene_color.y as u8, scene_color.z as u8]);
        }
    
        frame_buffer.save("image.png").unwrap();
    }
}