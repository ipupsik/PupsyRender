use crate::engine::camera::{*};
use crate::engine::render_context::{*};
use crate::engine::math::ray::{*};
use crate::engine::math::vector3::{*};
use crate::engine::scene::{*};
use image::{Rgb};

pub struct Renderer {

}

impl Renderer {
    fn sample_scene(ray : Ray, scene : &Scene) -> Vector3 {
        let mut success : bool = false;
        let mut min_t : f64 = 0.0;
        let mut min_position : Vector3 = ray.origin;

        for mesh in scene.meshes.iter() {
            let hit_option: Option<Vector3>  = mesh.hit(ray);
            if hit_option.is_some() {
                let hit_position: Vector3 = hit_option.unwrap();
                let t : f64 = (hit_position - ray.origin).length();
                success = true;
                if t < min_t {
                    min_t = t;
                    min_position = hit_position;
                }
            }
        }

        if !success {
            return ray.direction * 255.999;
        }
        min_position * 255.999
    }

    pub fn render(&self, camera : Camera, render_context : RenderContext) {
        let mut frame_buffer = render_context.render_target.frame_buffer;
        for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
            let u = x as f64 / (render_context.render_target.width - 1) as f64;
            let v = y as f64 / (render_context.render_target.height - 1) as f64;

            let world_position = Vector3{vec : [camera.width * (u - 1.0 / 2.0), camera.height * (v - 1.0 / 2.0), 0.0]};

            let scene_color : Vector3 = Self::sample_scene(Ray{origin : camera.ray.origin, direction : world_position - camera.ray.direction}, &render_context.scene);
    
            *pixel = Rgb([scene_color.x() as u8, scene_color.y() as u8, scene_color.z() as u8]);
        }
    
        frame_buffer.save("image.png").unwrap();
    }
}