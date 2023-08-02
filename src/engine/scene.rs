use std::vec;
use glam::{Vec2, Vec3A, Vec4, Mat4};
use gltf::json::camera::Type;
use image::GenericImageView;
use crate::engine::material::*;
use crate::engine::material::diffuse::*;
use crate::engine::material::metal::*;
use crate::engine::material::normal::*;
use crate::engine::material::refraction::*;
use crate::engine::material::pbr::*;
use crate::engine::material::pbr_metallic_roughness::*;
use crate::engine::material::uv::*;
use crate::engine::texture::texture2d::*;
use crate::engine::texture::*;
use crate::engine::bvh::node::*;

use super::geometry::sphere::*;
use super::geometry::traceable::Traceable;
use super::geometry::triangle::*;
use super::geometry::vertex::Vertex;

use std::io::Cursor;
use image::io::Reader as ImageReader;

use data_url::{DataUrl, mime};
//use crate::engine::octree::octree::*;

use std::rc::*;
use std::sync::{Arc};
use std::io;
use std::fs;

pub struct Scene {
    pub geometry: Vec<Arc<dyn Traceable>>,
    pub materials: Vec<Arc<dyn Material>>,
    pub textures: Vec<Arc<Texture>>,
    pub bvh: Node,
}

struct GLTFContext {
    pub decoded_buffers : Vec<Vec<u8>>,
    pub decoded_images : Vec<Texture>,
}

impl GLTFContext {
    pub fn new() -> Self {
        Self{
            decoded_buffers : Vec::new(),
            decoded_images : Vec::new(),
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            bvh: Node::new(&Vec::new(), 0, 0),
            geometry : Vec::new(),
            materials : Vec::new(),
            textures : Vec::new(),
        }
    }

    pub fn build_bvh(&mut self) {
        self.bvh = Node::new(&self.geometry, 0, self.geometry.len());
    }

    fn decode_triangle_vec3_indexed(
        buffer : &Vec<u8>, offset: usize, stride: usize, raw_size: usize,
        indices_buffer : &Vec<u8>, indices_offset : usize, indices_stride: usize,  indices_raw_size: usize
    ) -> [Vec3A; 3] {
        let index1 = Self::decode_int(indices_buffer, indices_offset, indices_raw_size) as usize;
        let index2 = Self::decode_int(indices_buffer, indices_offset + indices_stride, indices_raw_size) as usize;
        let index3 = Self::decode_int(indices_buffer, indices_offset + indices_stride * 2, indices_raw_size) as usize;

        let pos1 = Self::decode_vec3(buffer, offset + index1 * stride, raw_size);
        let pos2 = Self::decode_vec3(buffer, offset + index2 * stride, raw_size);
        let pos3 = Self::decode_vec3(buffer, offset + index3 * stride, raw_size);

        [pos1, pos2, pos3]
    }

    fn decode_triangle_vec3(buffer : &Vec<u8>, offset : usize, stride : usize, 
        raw_size : usize) -> [Vec3A; 3] {
        let pos1 = Self::decode_vec3(buffer, offset, raw_size);
        let pos2 = Self::decode_vec3(buffer, offset + stride, raw_size);
        let pos3 = Self::decode_vec3(buffer, offset + stride * 2, raw_size);

        [pos1, pos2, pos3]
    }

    fn decode_triangle_vec2_indexed(
        buffer : &Vec<u8>, offset: usize, stride: usize, raw_size: usize,
        indices_buffer : &Vec<u8>, indices_offset : usize, indices_stride: usize,  indices_raw_size: usize
    ) -> [Vec2; 3] {
        let index1 = Self::decode_int(indices_buffer, indices_offset, indices_raw_size) as usize;
        let index2 = Self::decode_int(indices_buffer, indices_offset + indices_stride, indices_raw_size) as usize;
        let index3 = Self::decode_int(indices_buffer, indices_offset + indices_stride * 2, indices_raw_size) as usize;

        let uv1 = Self::decode_vec2(buffer, offset + index1 * stride, raw_size);
        let uv2 = Self::decode_vec2(buffer, offset + index2 * stride, raw_size);
        let uv3 = Self::decode_vec2(buffer, offset + index3 * stride, raw_size);

        [uv1, uv2, uv3]
    }

    fn decode_triangle_vec2(buffer : &Vec<u8>, offset : usize, stride : usize, 
        raw_size : usize) -> [Vec2; 3] {
        let uv1 = Self::decode_vec2(buffer, offset, raw_size);
        let uv2 = Self::decode_vec2(buffer, offset + stride, raw_size);
        let uv3 = Self::decode_vec2(buffer, offset + stride * 2, raw_size);

        [uv1, uv2, uv3]
    }

    fn decode_vec3(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> Vec3A {
        return Vec3A::new(
            f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid x")),
            f32::from_le_bytes(buffer[offset + raw_size..offset + raw_size * 2].try_into().expect("Invalid y")),
            f32::from_le_bytes(buffer[offset + raw_size * 2..offset + raw_size * 3].try_into().expect("Invalid z"))
        );
    }

    fn decode_vec2(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> Vec2 {
        return Vec2::new(
            f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid x")),
            f32::from_le_bytes(buffer[offset + raw_size..offset + raw_size * 2].try_into().expect("Invalid y"))
        );
    }

    fn decode_scalar(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> f32 {
        return f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid scalar"));
    }

    fn decode_int(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> u32 {
        if raw_size == 1 {
            return u8::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index")) as u32;
        } else if raw_size == 2 {
            return u16::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index")) as u32;
        } else if raw_size == 4 {
            return u32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index"));
        }
        return 0;
    }

    fn load_gltf_material(&mut self, context : &GLTFContext, material: gltf::material::Material) -> Arc<dyn Material> {
        let mut pbr_material = PBRMaterial::new();

        let pbr_metallic_roughness = material.pbr_metallic_roughness();
        let pbr_base_color_texture_option = pbr_metallic_roughness.base_color_texture();
        if pbr_base_color_texture_option.is_some() {
            let pbr_base_color_texture = pbr_base_color_texture_option.unwrap();
            let image = &context.decoded_images[pbr_base_color_texture.texture().index()];
            pbr_material.pbr_metallic_roughness.base_color_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let pbr_base_color_factor = pbr_metallic_roughness.base_color_factor();
        pbr_material.pbr_metallic_roughness.base_color_factor = Vec4::from(pbr_base_color_factor);

        let pbr_metalic_roughness_texture_option = pbr_metallic_roughness.metallic_roughness_texture();
        if pbr_metalic_roughness_texture_option.is_some() {
            let pbr_metalic_roughness_texture = pbr_metalic_roughness_texture_option.unwrap();
            let image = &context.decoded_images[pbr_metalic_roughness_texture.texture().index()];
            pbr_material.pbr_metallic_roughness.metalic_roughness_texture = Arc::new(Texture2D::new(image.clone()));
        }

        pbr_material.pbr_metallic_roughness.metalic_factor = pbr_metallic_roughness.metallic_factor();
        pbr_material.pbr_metallic_roughness.roughness_factor = pbr_metallic_roughness.roughness_factor();

        let normal_texture_option = material.normal_texture();
        if normal_texture_option.is_some() {
            let normal_texture = normal_texture_option.unwrap();
            let image = &context.decoded_images[normal_texture.texture().index()];
            pbr_material.normal_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let occlusion_texture_option = material.occlusion_texture();
        if occlusion_texture_option.is_some() {
            let occlusion_texture = occlusion_texture_option.unwrap();
            let image = &context.decoded_images[occlusion_texture.texture().index()];
            pbr_material.occlusion_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let emissive_texture_option = material.emissive_texture();
        if emissive_texture_option.is_some() {
            let emissive_texture = emissive_texture_option.unwrap();
            let image = &context.decoded_images[emissive_texture.texture().index()];
            pbr_material.emissive_texture = Arc::new(Texture2D::new(image.clone()));
        }
        pbr_material.emissive_factor = Vec3A::from(material.emissive_factor());

        Arc::new(pbr_material)
    }

    fn load_gltf_node(&mut self, context : &GLTFContext, node: &gltf::Node, matrix: &Mat4) {
        let node_transform_matrix = node.transform().matrix();
        let new_matrix = matrix.mul_mat4(&Mat4::from_cols_array_2d(&node_transform_matrix));

        let mesh_option =  node.mesh();
        if mesh_option.is_some() {
            let gltf_mesh = mesh_option.unwrap();

            for primitive in gltf_mesh.primitives() {
                let mut positions: Vec<[Vec3A; 3]> = Vec::new();
                let mut uvs: Vec<[Vec2; 3]> = Vec::new();
                let mut normals: Vec<[Vec3A; 3]> = Vec::new();

                for attribute in primitive.attributes() {
                    let sparse_option = attribute.1.sparse();
                    let buffer_view_option = attribute.1.view();
                    let raw_type = attribute.1.data_type();
                    let data_type = attribute.1.dimensions();

                    let primitive_indices_option = primitive.indices();
                    let buffer_view = buffer_view_option.expect("Error in gltf file, buffer view is empty");
                    let buffer = &context.decoded_buffers[buffer_view.buffer().index()];
                    let size = data_type.multiplicity() * raw_type.size();
                    let mut stride = size; 
                    if buffer_view.stride().is_some() {
                        stride = buffer_view.stride().unwrap();
                    }

                    if sparse_option.is_some() {
                        let sparse = sparse_option.unwrap();

                        let indices_buffer_view = sparse.indices().view();
                        let indices_buffer = &context.decoded_buffers[indices_buffer_view.buffer().index()];
                        let indices_size = sparse.indices().index_type().size();
                        let mut indices_stride = indices_size; 
                        if indices_buffer_view.stride().is_some() {
                            indices_stride = indices_buffer_view.stride().unwrap();
                        }

                        let mut indices_buffer_pos = 0;
                        while indices_buffer_pos < indices_buffer_view.length() {
                            let pos_raw_indices_data_pos = indices_buffer_view.offset() + indices_buffer_pos;
                            match attribute.0 {
                                gltf::Semantic::Positions | gltf::Semantic::Normals => {
                                    let mut decoded_triangle = Self::decode_triangle_vec3_indexed(
                                        buffer, buffer_view.offset(), stride, raw_type.size(),
                                        indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_size
                                    );

                                    decoded_triangle.iter_mut().for_each(|vec| *vec = Vec3A::from(new_matrix.mul_vec4(Vec4::from((*vec, 0.0)))));

                                    match attribute.0 {
                                        gltf::Semantic::Positions => positions.push(decoded_triangle),
                                        gltf::Semantic::Normals => normals.push(decoded_triangle),
                                        _ => println!("Invalid attribute"),
                                    };
                                },
                                gltf::Semantic::TexCoords(set) => {
                                    let decoded_triangle = Self::decode_triangle_vec2_indexed(
                                        buffer, buffer_view.offset(), stride, raw_type.size(),
                                        indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_size
                                    );
                                    uvs.push(decoded_triangle);
                                },
                                _ => {
                                    println!("Unhandled attribute");
                                    break;
                                },
                            };

                            indices_buffer_pos += indices_stride * 3;
                        }
                    } else {
                        if primitive_indices_option.is_some() {
                            let indices = primitive_indices_option.unwrap();
                            let indices_buffer_view = indices.view().expect("Error in gltf file, indices buffer view is empty, when
                                indices are not");
                            let indices_buffer = &context.decoded_buffers[indices_buffer_view.buffer().index()];
        
                            let indices_raw_type = indices.data_type();
                            let indices_data_type = indices.dimensions();
        
                            let indices_size = indices_data_type.multiplicity() * indices_raw_type.size();
                            let mut indices_stride = indices_size; 
                            if indices_buffer_view.stride().is_some() {
                                indices_stride = indices_buffer_view.stride().unwrap();
                            }

                            let mut indices_buffer_pos = 0;
                            while indices_buffer_pos < indices_buffer_view.length() {
                                let pos_raw_indices_data_pos = indices_buffer_view.offset() + indices_buffer_pos;
                                match attribute.0 {
                                    gltf::Semantic::Positions | gltf::Semantic::Normals => {
                                        let mut decoded_triangle = Self::decode_triangle_vec3_indexed(
                                            buffer, buffer_view.offset(), stride, raw_type.size(),
                                            indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_raw_type.size()
                                        );

                                        decoded_triangle.iter_mut().for_each(|vec| *vec = Vec3A::from(new_matrix.mul_vec4(Vec4::from((*vec, 0.0)))));

                                        match attribute.0 {
                                            gltf::Semantic::Positions => positions.push(decoded_triangle),
                                            gltf::Semantic::Normals => normals.push(decoded_triangle),
                                            _ => println!("Invalid attribute"),
                                        };
                                    },
                                    gltf::Semantic::TexCoords(set) => {
                                        let decoded_triangle = Self::decode_triangle_vec2_indexed(
                                            buffer, buffer_view.offset(), stride, raw_type.size(),
                                            indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_raw_type.size()
                                        );

                                        uvs.push(decoded_triangle);
                                    },
                                    _ => {
                                        println!("Unhandled attribute");
                                        break;
                                    },
                                };

                                indices_buffer_pos += indices_stride * 3;
                            }
                        } else {
                            let mut buffer_pos = 0;
                            while buffer_pos < buffer_view.length() {
                                let pos_raw_data_pos = buffer_view.offset() + buffer_pos;   

                                match attribute.0 {
                                    gltf::Semantic::Positions | gltf::Semantic::Normals => {
                                        let mut decoded_triangle =  Self::decode_triangle_vec3(
                                            buffer, pos_raw_data_pos, stride, raw_type.size()
                                        );

                                        decoded_triangle.iter_mut().for_each(|vec| *vec = Vec3A::from(new_matrix.mul_vec4(Vec4::from((*vec, 0.0)))));

                                        match attribute.0 {
                                            gltf::Semantic::Positions => positions.push(decoded_triangle),
                                            gltf::Semantic::Normals => normals.push(decoded_triangle),
                                            _ => println!("Invalid attribute"),
                                        };
                                    },
                                    gltf::Semantic::TexCoords(set) => {
                                        let decoded_triangle = Self::decode_triangle_vec2(
                                            buffer, pos_raw_data_pos, stride, raw_type.size()
                                        );
                                        uvs.push(decoded_triangle);
                                    },
                                    _ => {
                                        println!("Unhandled attribute");
                                        break;
                                    },
                                }                                     

                                buffer_pos += stride * 3;
                            }
                        }                      
                    }
                }

                let material = self.load_gltf_material(context, primitive.material());

                assert!(positions.len() == uvs.len());
                assert!(uvs.len() == normals.len());
                let triangles_count = positions.len();

                let mut mesh_triangles: Vec<Arc<Box<dyn Traceable>>> = Vec::new();
                mesh_triangles.reserve(triangles_count);

                for i in 0..triangles_count {
                    let vertex1 = Vertex::new(positions[i][0], normals[i][0], uvs[i][0]);
                    let vertex2 = Vertex::new(positions[i][1], normals[i][1], uvs[i][1]);
                    let vertex3 = Vertex::new(positions[i][2], normals[i][2], uvs[i][2]);

                    self.geometry.push(Arc::new(Triangle::new(material.clone(), vertex1, vertex2, vertex3)));
                }

                self.materials.push(material);
            }
        }

        for child in node.children() {
            self.load_gltf_node(context, &child, &new_matrix);
        }
    }

    pub fn load_gltf(&mut self, path: &str) {
        let file = fs::File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();
        
        let mut context = GLTFContext::new();
        context.decoded_buffers.resize(gltf.buffers().count(), Vec::new());
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(data) => {
                    let url = DataUrl::process(data).unwrap();
                    (context.decoded_buffers[buffer.index()], _) = url.decode_to_vec().unwrap();
                },
                gltf::buffer::Source::Bin => println!("Engine does not support binary buffer format"),
            }
        }

        context.decoded_images.resize(gltf.images().count(), Texture::new(0, 0, Arc::new(Vec::new())));
        for image in gltf.images() {
            let mut image_raw_data = Vec::new();

            match image.source() {
                gltf::image::Source::Uri{ uri, mime_type } => {
                    let url = DataUrl::process(uri).unwrap();
                    (image_raw_data, _) = url.decode_to_vec().unwrap();
                },
                gltf::image::Source::View { view, mime_type } => {
                    let buffer = &context.decoded_buffers[view.buffer().index()];
                    image_raw_data = buffer[view.offset()..view.offset() + view.length()].to_vec();
                },
            }

            let image_reader = ImageReader::new(Cursor::new(image_raw_data));
            match image_reader.with_guessed_format() {
                Ok(value) => {
                    match value.decode() {
                        Ok(value) => {
                            context.decoded_images[image.index()] = Texture::new(
                                value.width(),
                                value.height(),
                                Arc::new(value.into_bytes())
                            );
                        },
                        Err(error) => {
                            println!("Failed to decode image; {}", error.to_string());
                        }
                    }
                },
                Err(error) => {
                    println!("Failed to guess format of a texture; {}", error.to_string());
                }
            }
        }

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                self.load_gltf_node(&context, &node, &Mat4::IDENTITY);
            }
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material = Arc::new(DiffuseMaterial{});
        let metal_material = Arc::new(
            MetalMaterial{metalness : 0.9}
        );
        let normal_material = Arc::new(NormalMaterial{});
        let refraction_material = Arc::new(
            RefractionMaterial{refraction_type: RefractionType::Glass}
        );
        let uv_material = Arc::new(
            UVMaterial{}
        );

        self.geometry.push(Arc::new(Sphere{material: diffuse_material.clone(), radius : 0.5, position : Vec3A::new(1.7, 0.0, 0.6)}));
        self.geometry.push(Arc::new(Sphere{material: diffuse_material.clone(), radius : 100.0, position : Vec3A::new(0.0, -101.0, 1.0)}));
        self.geometry.push(Arc::new(Sphere{material: metal_material.clone(), radius : 0.5, position : Vec3A::new(1.0, 0.0, 1.2)}));
        self.geometry.push(Arc::new(Sphere{material: normal_material.clone(), radius : 0.5, position : Vec3A::new(-1.0, 0.0, 1.2)}));
        self.geometry.push(Arc::new(Sphere{material: refraction_material.clone(), radius : 0.5, position : Vec3A::new(-1.3, 0.15, 0.5)}));

        self.materials.push(diffuse_material.clone());
        self.materials.push(metal_material.clone());
        self.materials.push(normal_material.clone());
        self.materials.push(refraction_material.clone());
        self.materials.push(uv_material.clone());

        // gltf
        //self.load_gltf("example1.gltf");
        self.load_gltf("example2.gltf");
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}