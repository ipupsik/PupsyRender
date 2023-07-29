use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::scene::{*};
use crate::engine::geometry::traceable::HitResult;
use image::{Rgb};

pub struct Renderer {

}

impl Renderer {
    fn sample_scene(ray : Ray, scene : &Scene) -> Vector3 {
        let mut success = false;
        let mut min_hit_result = HitResult::new();
        for mesh in scene.meshes.iter() {
            let hit_option: Option<HitResult>  = mesh.hit(ray);
            if hit_option.is_some() {
                let hit_result = hit_option.unwrap();
                if hit_result.t < min_hit_result.t {
                    success = true;
                    min_hit_result = hit_result
                }
            }
        }

        if !success {
            return (ray.direction.y() + 0.5) * Vector3::new(0.5, 0.7, 0.1) * 255.999;
        }
        (min_hit_result.normal + Vector3::new(1.0, 1.0, 1.0)) * 0.5 * 255.999
    }

    pub fn render(&self, camera : Camera, render_context : RenderContext) {
        let mut frame_buffer = render_context.render_target.frame_buffer;
        for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
            let u = x as f64 / (render_context.render_target.width - 1) as f64;
            let v = y as f64 / (render_context.render_target.height - 1) as f64;

            let world_position = Vector3{vec : [camera.width * (u - 1.0 / 2.0), camera.height * (v - 1.0 / 2.0), 0.0]};

            let ray_direction : Vector3 = world_position - camera.ray.direction;
            ray_direction.normalize();

            let scene_color : Vector3 = Self::sample_scene(Ray{origin : camera.ray.origin, direction : ray_direction}, &render_context.scene);
    
            *pixel = Rgb([scene_color.x() as u8, scene_color.y() as u8, scene_color.z() as u8]);
        }
    
        frame_buffer.save("image.png").unwrap();
    }
}