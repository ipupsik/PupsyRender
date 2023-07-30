use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
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
    fn sample_scene(ray : Ray, scene : &Scene, depth : u64) -> Vector3 {
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
            let t = 0.5 * (ray.direction.y() + 1.0);
            return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        }

        if depth > 1 {
            let brdf_vector = unsafe {(*min_hit_result.material.as_ptr()).scatter.scatter(ray, &min_hit_result)};

            let new_ray = Ray{origin : min_hit_result.position, direction : brdf_vector};
            return Self::sample_scene(new_ray, scene, depth - 1) * 0.8;
        }

       Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn render(&self, camera : Camera, render_context : RenderContext) {
        let mut frame_buffer = render_context.render_target.frame_buffer;
        for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
            let mut scene_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..render_context.spp {
                let u = (x as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.width - 1) as f64;
                let v = (y as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (render_context.render_target.height - 1) as f64;

                let ray = camera.get_ray(u, 1.0 - v);

                let mut current_sample = Self::sample_scene(ray, 
                    &render_context.scene, render_context.max_depth);

                // Divide the color by the number of samples and gamma-correct for gamma=2.0.
                current_sample.vec[0] = current_sample.x().powf(1.0 / 2.2);
                current_sample.vec[1] = current_sample.y().powf(1.0 / 2.2);
                current_sample.vec[2] = current_sample.z().powf(1.0 / 2.2);

                current_sample = current_sample * 255.999;

                scene_color = scene_color + current_sample / render_context.spp as f64;
            }
    
            *pixel = Rgb([scene_color.x() as u8, scene_color.y() as u8, scene_color.z() as u8]);
        }
    
        frame_buffer.save("image.png").unwrap();
    }
}