use crate::engine::camera::*;
use crate::engine::render_context::*;
use crate::engine::math::ray::*;
use glam::{Vec3A};
use gltf::mesh::util::weights;
use crate::engine::geometry::bvh::node::*;
use crate::engine::material::pdf::cosine::*;
use rand::Rng;

use image::{ImageBuffer};
use std::rc::Rc;
use std::sync::{Arc};
use image::{Rgb};

use super::geometry::traceable::Traceable;
use super::material::pdf::PDF;
use super::material::pdf::mix::MixPDF;
use super::material::pdf::traceable::GeometryPDF;

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

    fn sample_scene(ray : &Ray, bvh : &Node, lights: &Vec<Arc<dyn Traceable>>, depth : u32) -> Vec3A {
        let mut ray = ray.clone();
        let mut average_sample = Vec3A::ONE;
        for i in 0..depth {
            let hit_result_option = bvh.hit(&ray, 0.001, f32::MAX);

            if !hit_result_option.is_some() {
                let t = 0.5 * (ray.direction.y + 1.0);
                let sky = (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0);
                return average_sample * sky;
            }

            let hit_result = hit_result_option.unwrap();

            let scatter_result = hit_result.material.scatter(&ray, &hit_result);
            let emmission = hit_result.material.emit(&ray, &hit_result);

            let mut sample = scatter_result.attenuation;

            if scatter_result.scatter.is_some() {
                let pdf = scatter_result.scatter.unwrap();

                let mut scatter = Vec3A::ZERO;
                let mut pdf_value = 0.0;

                if lights.len() > 0 {
                    let mut pdfs: Vec<Rc::<dyn PDF>> = Vec::new();
                    let mut weights = Vec::new();

                    let uniform_weight = 1.0 / lights.len() as f32;

                    for light in lights {
                        pdfs.push(Rc::new(GeometryPDF{origin: hit_result.position, geometry: light.clone()}));
                        weights.push(uniform_weight);
                    }

                    let light_pdf = Rc::new(MixPDF{ 
                        pdfs: pdfs,
                        weights: weights});

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

                let mut scattering_ray = Ray{origin : hit_result.position, direction : scatter};
                let mut scattering_pdf_value = hit_result.material.scattering_pdf(&ray, &hit_result, &scattering_ray);

                if scatter_result.alpha_masked {
                    scattering_ray = Ray{origin : hit_result.position, direction : ray.direction};
                    scattering_pdf_value = 1.0;
                    pdf_value = 1.0;
                }

                sample = sample * scattering_pdf_value / pdf_value;

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
    
        struct WorkerInfo {
            pub back_buffer_chunk: Vec<Vec3A>,
            pub sample_index: u32,
        }
        let mut output_frame_buffer: Vec<WorkerInfo> = Vec::new();
        let mut rgb_frame_buffer = ImageBuffer::new(width, height);
        
        // Parallel our work
        let frame_buffer_len = (width * height) as usize;

        //let num_cores = 1;
        let num_cores = num_cpus::get();
        let chunk_size = frame_buffer_len / num_cores;

        let mut threads = Vec::with_capacity(num_cores);

        let (tx, rx) =  mpsc::channel();

        for thr in 0..num_cores
        {
            let index_begin = thr * chunk_size;
            let mut index_end = (thr + 1) * chunk_size;
            if thr == num_cores - 1
            {
                index_end = frame_buffer_len;
            }

            output_frame_buffer.push(WorkerInfo{back_buffer_chunk: Vec::new(), sample_index: 0});
            output_frame_buffer[thr].back_buffer_chunk.resize((index_end - index_begin) as usize, Vec3A::ZERO);

            let camera = Arc::new(camera.clone());
            let render_context = Arc::new(render_context.clone());

            let tx =  tx.clone();
            threads.push(
                thread::spawn(move || {
                    let mut frame_buffer_slice = vec![Vec3A::ZERO; index_end - index_begin];
                    for sample_index in 0..render_context.spp {
                        for i in 0..index_end - index_begin
                        {
                            let x = (index_begin + i) as u32 % width;
                            let y = (index_begin + i) as u32 / width;

                            let u = (x as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (width - 1) as f32;
                            let v = (y as f32 + rand::thread_rng().gen_range(0.0..1.0)) / (height - 1) as f32;
            
                            let ray = camera.get_ray(u, 1.0 - v);
            
                            let current_sample = Self::sample_scene(&ray, 
                                &render_context.scene.bvh, &render_context.scene.lights, render_context.max_depth);
        
                            frame_buffer_slice[i] += current_sample / render_context.spp as f32;
                        }
                        tx.send((frame_buffer_slice.clone(), thr, sample_index)).unwrap();
                    }
                })
            );
        }

        for sample_index in 0..render_context.spp {
            for v in rx.iter().take(num_cores){
                let (frame_buffer_slice, thread_index, sample_index) = v;
                output_frame_buffer[thread_index].back_buffer_chunk = frame_buffer_slice;
                output_frame_buffer[thread_index].sample_index = sample_index;
            }

            for (x, y, pixel) in rgb_frame_buffer.enumerate_pixels_mut() {
                *pixel = Rgb([0, 0, 0]);
                let mut linear_index = (y * width + x) as usize;
                for chunk in output_frame_buffer.iter() {
                    if linear_index < chunk.back_buffer_chunk.len() {
                        let mut scene_color = chunk.back_buffer_chunk[linear_index];
        
                        //scene_color = Self::tone_mapping(scene_color);
                        scene_color *= render_context.spp as f32 / (chunk.sample_index as f32 + 1.0);
                        scene_color = Self::gamma_correction(scene_color);  
                        scene_color *= 255.0;
    
                        *pixel = Rgb([scene_color.x as u8, scene_color.y as u8, scene_color.z as u8]);
                        break;
                    }
                    linear_index -= chunk.back_buffer_chunk.len();
                }
            }
        
            rgb_frame_buffer.save(format!("example/{}.png", camera.name())).unwrap();
        }

        for thr in threads
        {
            thr.join().unwrap();
        }
    }
}