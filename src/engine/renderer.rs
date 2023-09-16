use crate::engine::camera::*;
use crate::engine::render_context::*;
use crate::engine::math::ray::*;
use crate::engine::scene::*;
use glam::{Vec3A};
use gltf::mesh::util::weights;
use workerpool::Worker;
use crate::engine::geometry::bvh::node::*;
use crate::engine::material::pdf::cosine::*;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};
use rand::Rng;

use image::{ImageBuffer};
use std::rc::Rc;
use std::sync::{Arc};
use image::{Rgb};

use super::geometry::traceable::Traceable;
use super::material::pdf::PDF;
use super::material::pdf::mix::MixPDF;
use super::material::pdf::traceable::GeometryPDF;
use super::profile::Profile;
use super::profile::ProfileType;

use std::thread::{self};
use std::sync::*;

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

    fn sample_scene(ray : &Ray, scene: &Scene, depth : u32) -> Vec3A {
        let mut ray = ray.clone();
        let mut average_sample = Vec3A::ONE;

        for _ in 0..depth {
            let (hit_result_option, traceable) = scene.bvh.hit(&ray, 0.001, f32::MAX);

            if !hit_result_option.is_some() {
                let t = 0.5 * (ray.direction.y + 1.0);
                let sky = (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.9, 0.3, 0.3);

                return average_sample * (sky);
            }

            let hit_result = &hit_result_option.unwrap();

            let mut pdfs: Vec<Rc::<dyn PDF>> = Vec::new();
            let mut weights = Vec::new();

            let uniform_weight = 1.0 / scene.lights.len() as f32;

            for light in scene.lights.iter() {
                pdfs.push(Rc::new(GeometryPDF{origin: hit_result.position, geometry: light.clone()}));
                weights.push(uniform_weight);
            }

            let light_pdf = Rc::new(MixPDF{ 
                pdfs: pdfs,
                weights: weights});

            let scatter_result = traceable.material().scatter(&ray, &hit_result);

            let emmission = traceable.material().emit(&ray, &scatter_result.hit_result);
            let mut sample = scatter_result.attenuation;

            if scatter_result.scatter.is_some() {
                let pdf = &scatter_result.scatter.unwrap();

                let mut scatter = Vec3A::ZERO;
                let mut pdf_value = 0.0;

                if scene.lights.len() > 0 {
                    let final_pdf = MixPDF{ 
                        pdfs: vec![pdf.clone(), light_pdf.clone()],
                        weights: vec![0.5, 0.5]};

                    scatter = final_pdf.generate();
                    pdf_value = final_pdf.value(scatter);
                }
                else {
                    scatter = pdf.generate();
                    pdf_value = pdf.value(scatter);
                }

                let mut scattering_ray = Ray{origin : scatter_result.hit_result.position, direction : scatter};

                if scatter_result.alpha_masked {
                    scattering_ray = Ray{origin : scatter_result.hit_result.position, direction : ray.direction};
                    pdf_value = 1.0;
                }

                sample = sample / pdf_value;

                ray = scattering_ray;
            } else {
                return average_sample * emmission;
            }

            average_sample = emmission + average_sample * sample;
        }

        //sample
        Vec3A::ZERO
        //average_sample
    }

    pub fn render(&self, camera: Arc<PerspectiveCamera>, render_context : Arc<RenderContext>) {
        let height: u32 = render_context.resolution;
        let width: u32 = (height as f32 * camera.aspect_ratio()) as u32;
    
        struct WorkerInput {
            pub task_index: usize,
            pub x: usize,
            pub y: usize,
            pub height: u32,
            pub width: u32,
            pub render_context: Arc<RenderContext>,
            pub camera: Arc<PerspectiveCamera>
        }

        #[derive(Clone)]
        #[derive(Copy)]
        struct WorkerOutput {
            pub task_index: usize,
            pub tile: [[Vec3A; TILE_SIZE]; TILE_SIZE],
            pub sample_index: u32,
        }

        impl WorkerOutput {
            pub fn new() -> Self {
                WorkerOutput{
                    task_index: 0,
                    tile: [[Vec3A::ZERO; TILE_SIZE]; TILE_SIZE], 
                    sample_index: 0
                }
            }
        }

        struct RenderTask {
           
        }
        impl Default for RenderTask {
            fn default() -> Self {
                Self {
                   
                }
            }
        }
        impl Worker for RenderTask {
            type Input = WorkerInput;
            type Output = WorkerOutput;
        
            fn execute(&mut self, inp: Self::Input) -> Self::Output {
                let mut output = WorkerOutput::new();

                for local_y in 0..TILE_SIZE {
                    if inp.y + local_y > inp.height as usize {
                        break;
                    }
                    for local_x in 0..TILE_SIZE {
                        if inp.x + local_x > inp.width as usize {
                            break;
                        }
                        let mut current_color = Vec3A::ZERO;
                        for sample_index in 0..inp.render_context.spp {
                            let x = inp.x + local_x;
                            let y = inp.y + local_y;

                            let u = (x as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (inp.width - 1) as f32;
                            let v = (y as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (inp.height - 1) as f32;
            
                            let ray = inp.camera.get_ray(u, 1.0 - v);
            
                            let mut current_sample = Renderer::sample_scene(&ray, 
                                &inp.render_context.scene, inp.render_context.max_depth);
        
                            if current_sample.x.is_nan() {
                                current_sample.x = 1.0;
                            }
                            if current_sample.y.is_nan() {
                                current_sample.y = 1.0;
                            }
                            if current_sample.z.is_nan() {
                                current_sample.z = 1.0;
                            }

                            current_color += current_sample / inp.render_context.spp as f32;
                        }
                        output.tile[local_x][local_y] = current_color;
                    }
                }

                output.sample_index = inp.render_context.spp - 1;
                output.task_index = inp.task_index;
                output
            }
        }

        let pool = Pool::<RenderTask>::new(num_cpus::get());
        let mut tile_x: usize = (width / TILE_SIZE as u32) as usize;
        tile_x += if width as usize % TILE_SIZE > 0 {1} else {0};

        let mut tile_y: usize = (height / TILE_SIZE as u32) as usize;
        tile_y += if height as usize % TILE_SIZE > 0 {1} else {0};

        let num_tasks = tile_x * tile_y;

        let mut output_frame_buffer: Vec<WorkerOutput> = vec![WorkerOutput::new(); num_tasks];

        let (tx, rx) =  mpsc::channel();

        for tile_x_index in 0..tile_x
        {
            for tile_y_index in 0..tile_y
            {
                let inp = WorkerInput{
                    task_index: tile_x_index + tile_y_index * tile_x,
                    x: tile_x_index * TILE_SIZE,
                    y: tile_y_index * TILE_SIZE,
                    height: height,
                    width: width,
                    render_context: render_context.clone(),
                    camera: camera.clone(),
                };
                pool.execute_to(tx.clone(), inp);
            }
        }

        for output in rx.iter().take(num_tasks){
            output_frame_buffer[output.task_index] = output;
        }

        let mut rgb_frame_buffer = ImageBuffer::new(width, height);

        for (x, y, pixel) in rgb_frame_buffer.enumerate_pixels_mut() {
            *pixel = Rgb([0, 0, 0]);
            let tile_x_index: usize = (x / TILE_SIZE as u32) as usize;
            let tile_y_index: usize = (y / TILE_SIZE as u32) as usize;
            let local_tile_x_index: usize = (x % TILE_SIZE as u32) as usize;
            let local_tile_y_index: usize = (y % TILE_SIZE as u32) as usize;

            let chunk = &output_frame_buffer[tile_y_index * tile_x + tile_x_index];

            let mut scene_color = chunk.tile[local_tile_x_index][local_tile_y_index];
    
            //scene_color = Self::tone_mapping(scene_color);
            scene_color *= render_context.spp as f32 / (chunk.sample_index as f32 + 1.0);
            scene_color = Self::gamma_correction(scene_color);  
            scene_color *= 255.0;

            *pixel = Rgb([scene_color.x as u8, scene_color.y as u8, scene_color.z as u8]);
        }
    
        rgb_frame_buffer.save(render_context.output.as_str()).unwrap();
    }
}