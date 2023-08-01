use std::vec;
use crate::engine::mesh::Mesh;
use glam::{Vec2, Vec3A, Vec4, Mat4};
use gltf::json::camera::Type;
use crate::engine::material::{*};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};
use crate::engine::material::normal::{*};
use crate::engine::material::refraction::{*};
use crate::engine::material::uv::{*};

use super::geometry::sphere::{*};
use super::geometry::traceable::Traceable;
use super::geometry::triangle::{*};
use super::geometry::vertex::Vertex;

use data_url::{DataUrl, mime};

use std::rc::{*};
use std::sync::{Arc};
use std::io;
use std::fs;

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Arc<Box<dyn Material>>>,
}

struct GLTFContext {
    pub decoded_buffers : Vec<Vec<u8>>,
    pub decoded_images : Vec<Vec<u8>>,
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
    pub const fn new() -> Self {
        Self { 
            meshes : Vec::new(),
            materials : Vec::new(),
        }
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

    fn load_gltf_material(&mut self, material: gltf::material::Material) -> Arc<Box<dyn Material>> {
        let pbr_metallic_roughness = material.pbr_metallic_roughness();
        let pbr_base_color_texture_option = pbr_metallic_roughness.base_color_texture();
        let pbr_base_color_factor = pbr_metallic_roughness.base_color_factor();
        let pbr_metalic_roughness_texture_option = pbr_metallic_roughness.metallic_roughness_texture();
        let pbr_metalic_factor = pbr_metallic_roughness.metallic_factor();
        let pbr_roughness_factor = pbr_metallic_roughness.roughness_factor();

        let normal_texture_option = material.normal_texture();
        let occlusion_texture_option = material.occlusion_texture();
        let emissive_texture_option = material.emissive_texture();
        let emissive_factor = material.emissive_factor();

        Arc::new(Box::new(NormalMaterial{}))
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
                                    let decoded_vector = Self::decode_triangle_vec3_indexed(
                                        buffer, buffer_view.offset(), stride, raw_type.size(),
                                        indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_size
                                    );
                                    match attribute.0 {
                                        gltf::Semantic::Positions => positions.push(decoded_vector),
                                        gltf::Semantic::Normals => normals.push(decoded_vector),
                                        _ => println!("Invalid attribute"),
                                    };
                                },
                                gltf::Semantic::TexCoords(set) => {
                                    let decoded_vector = Self::decode_triangle_vec2_indexed(
                                        buffer, buffer_view.offset(), stride, raw_type.size(),
                                        indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_size
                                    );
                                    uvs.push(decoded_vector);
                                },
                                _ => println!("Invalid attribute"),
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
                                        let decoded_vector = Self::decode_triangle_vec3_indexed(
                                            buffer, buffer_view.offset(), stride, raw_type.size(),
                                            indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_raw_type.size()
                                        );
                                        match attribute.0 {
                                            gltf::Semantic::Positions => positions.push(decoded_vector),
                                            gltf::Semantic::Normals => normals.push(decoded_vector),
                                            _ => println!("Invalid attribute"),
                                        };
                                    },
                                    gltf::Semantic::TexCoords(set) => {
                                        let decoded_vector = Self::decode_triangle_vec2_indexed(
                                            buffer, buffer_view.offset(), stride, raw_type.size(),
                                            indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_raw_type.size()
                                        );
                                        uvs.push(decoded_vector);
                                    },
                                    _ => println!("Invalid attribute"),
                                };

                                indices_buffer_pos += indices_stride * 3;
                            }
                        } else {
                            let mut buffer_pos = 0;
                            while buffer_pos < buffer_view.length() {
                                let pos_raw_data_pos = buffer_view.offset() + buffer_pos;   

                                match attribute.0 {
                                    gltf::Semantic::Positions | gltf::Semantic::Normals => {
                                        let decoded_vector =  Self::decode_triangle_vec3(
                                            buffer, pos_raw_data_pos, stride, raw_type.size()
                                        );
                                        match attribute.0 {
                                            gltf::Semantic::Positions => positions.push(decoded_vector),
                                            gltf::Semantic::Normals => normals.push(decoded_vector),
                                            _ => println!("Invalid attribute"),
                                        };
                                    },
                                    gltf::Semantic::TexCoords(set) => {
                                        let decoded_vector = Self::decode_triangle_vec2(
                                            buffer, pos_raw_data_pos, stride, raw_type.size()
                                        );
                                        uvs.push(decoded_vector);
                                    },
                                    _ => println!("Invalid attribute"),
                                }                                     

                                buffer_pos += stride * 3;
                            }
                        }                      
                    }
                }

                let material = self.load_gltf_material(primitive.material());

                let mut mesh : Mesh = Mesh::new(material.clone());

                assert!(positions.len() == uvs.len());
                assert!(uvs.len() == normals.len());
                let triangles_count = positions.len();

                let mut mesh_triangles: Vec<Arc<Box<dyn Traceable>>> = Vec::new();
                mesh_triangles.reserve(triangles_count);

                for i in 0..triangles_count {
                    let vertex1 = Vertex::new(positions[i][0], normals[i][0], uvs[i][0]);
                    let vertex2 = Vertex::new(positions[i][1], normals[i][1], uvs[i][1]);
                    let vertex3 = Vertex::new(positions[i][2], normals[i][2], uvs[i][2]);

                    mesh.add_geometry(Arc::new(Triangle::new(vertex1, vertex2, vertex3)));
                }

                self.meshes.push(mesh);
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

        context.decoded_images.resize(gltf.images().count(), Vec::new());
        for image in gltf.images() {
            match image.source() {
                gltf::image::Source::Uri{ uri, mime_type } => {
                    let url = DataUrl::process(uri).unwrap();
                    (context.decoded_images[image.index()], _) = url.decode_to_vec().unwrap();
                },
                gltf::image::Source::View { view, mime_type } => println!("Engine does not support binary buffer format"),
            }
        }

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                self.load_gltf_node(&context, &node, &Mat4::IDENTITY);
            }
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material: Arc<Box<dyn Material>> = Arc::new(Box::new(DiffuseMaterial{}));
        let metal_material: Arc<Box<dyn Material>> = Arc::new(
            Box::new(MetalMaterial{metalness : 0.9})
        );
        let normal_material: Arc<Box<dyn Material>> = Arc::new(Box::new(NormalMaterial{}));
        let refraction_material: Arc<Box<dyn Material>> = Arc::new(
            Box::new(RefractionMaterial{refraction_type: RefractionType::Glass})
        );
        let uv_material: Arc<Box<dyn Material>> = Arc::new(
            Box::new(UVMaterial{})
        );

        let mut mesh : Mesh = Mesh::new(diffuse_material.clone());
        mesh.add_geometry(Arc::new(Sphere{radius : 0.5, position : Vec3A::new(1.7, 0.0, 0.6)}));
        mesh.add_geometry(Arc::new(Sphere{radius : 100.0, position : Vec3A::new(0.0, -100.5, 1.0)}));
        self.meshes.push(mesh);
        
        let mut mesh : Mesh = Mesh::new(metal_material.clone());
        mesh.add_geometry(Arc::new(Sphere{radius : 0.5, position : Vec3A::new(1.0, 0.0, 1.2)}));
        self.meshes.push(mesh);

        let mut mesh : Mesh = Mesh::new(normal_material.clone());
        mesh.add_geometry(Arc::new(Sphere{radius : 0.5, position : Vec3A::new(-1.0, 0.0, 1.2)}));
        self.meshes.push(mesh);

        let mut mesh : Mesh = Mesh::new(refraction_material.clone());
        mesh.add_geometry(Arc::new(Sphere{radius : 0.5, position : Vec3A::new(-1.3, 0.15, 0.5)}));
        self.meshes.push(mesh);

        self.materials.push(diffuse_material);
        self.materials.push(metal_material);
        self.materials.push(normal_material);
        self.materials.push(refraction_material);
        self.materials.push(uv_material);

        // gltf
        self.load_gltf("example1.gltf");
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}