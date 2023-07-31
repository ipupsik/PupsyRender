use std::vec;
use crate::engine::mesh::Mesh;
use glam::{Vec3A, Vec4, Mat4};
use gltf::json::camera::Type;
use crate::engine::material::{*};
use crate::engine::material::diffuse::{*};
use crate::engine::material::metal::{*};
use crate::engine::material::normal::{*};
use crate::engine::material::refraction::{*};

use super::geometry::sphere::{*};
use super::geometry::triangle::{*};
use super::geometry::vertex::Vertex;

use data_url::{DataUrl, mime};

use std::rc::{*};
use std::io;
use std::fs;

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

struct GLTFContext {
    pub decoded_buffers : Vec<Vec<u8>>,
}

impl GLTFContext {
    pub fn new() -> Self {
        Self{decoded_buffers : Vec::new()}
    }
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            meshes : Vec::new(),
        }
    }

    fn decode_vec3(buffer : &Vec<u8>, offset : usize) -> Vec3A {
        Vec3A::new(
            f32::from_be_bytes(buffer[offset..offset + 4].try_into().expect("Invalid [1] x coord")),
            f32::from_be_bytes(buffer[offset + 4..offset + 4 * 2].try_into().expect("Invalid [1] y coord")),
            f32::from_be_bytes(buffer[offset + 4 * 2..offset + 4 * 3].try_into().expect("Invalid [1] z coord"))
        )
    }

    fn load_gltf_node(&mut self, context : &GLTFContext, node: &gltf::Node, matrix: &Mat4) {
        let node_transform_matrix = node.transform().matrix();

        let new_matrix = matrix.mul_mat4(&Mat4::from_cols_array_2d(&node_transform_matrix));

        let mesh_option =  node.mesh();
        if mesh_option.is_some() {
            let gltf_mesh = mesh_option.unwrap();

            let diffuse_material_data = Rc::new(DiffuseMaterial{});
            let diffuse_material = Material{
                scatter : diffuse_material_data.clone(),
                sample: diffuse_material_data.clone()
            };
            let rc_diffuse_material = Rc::new(diffuse_material);

            let mut mesh : Mesh = Mesh::new(rc_diffuse_material.clone());

            for primitive in gltf_mesh.primitives() {
                for attribute in primitive.attributes() {
                    match attribute.0 {
                        gltf::Semantic::Positions => {
                            let raw_type = attribute.1.data_type();
                            let data_type = attribute.1.dimensions();
                            let buffer_view_option = attribute.1.view();
                            if buffer_view_option.is_some() {
                                let buffer_view = buffer_view_option.unwrap();
                                let buffer = &context.decoded_buffers[buffer_view.index()];
                                
                                let size = data_type.multiplicity() * raw_type.size();

                                let stride = size; 

                                let mut buffer_pos = 0;
                                while buffer_pos < buffer_view.length() {
                                    let mut pos_raw_data_pos = buffer_view.offset() + buffer_pos;

                                    let pos1_offset = 0;
                                    let pos1 = Self::decode_vec3(buffer, pos_raw_data_pos + pos1_offset);

                                    let pos2_offset = pos1_offset + 4 * 3;
                                    let pos2 = Self::decode_vec3(buffer, pos_raw_data_pos + pos2_offset);

                                    let pos3_offset = pos2_offset + 4 * 3;
                                    let pos3 = Self::decode_vec3(buffer, pos_raw_data_pos + pos3_offset);

                                    let vertex1 = Vertex::new(pos1);
                                    let vertex2 = Vertex::new(pos2);
                                    let vertex3 = Vertex::new(pos3);

                                    mesh.add_geometry(Rc::new(Triangle::new(vertex1, vertex2, vertex3)));

                                    buffer_pos += stride;
                                }
                            }
                        },
                        _ => println!("Unhandled semantic")
                    }
                }

                let indices_option = primitive.indices();
                if indices_option.is_some() {
                    let indices = indices_option.unwrap();
                }
                let material = primitive.material();
            }

            self.meshes.push(mesh);
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


        for scene in gltf.scenes() {
            for node in scene.nodes() {
                self.load_gltf_node(&context, &node, &Mat4::IDENTITY);
            }
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material_data = Rc::new(DiffuseMaterial{});
        let diffuse_material = Material{
            scatter : diffuse_material_data.clone(),
            sample: diffuse_material_data.clone()
        };
        let rc_diffuse_material = Rc::new(diffuse_material);

        let metal_material_data = Rc::new(MetalMaterial{metalness : 0.9});
        let metal_material = Material{
            scatter : metal_material_data.clone(),
            sample: metal_material_data.clone()
        };
        let rc_metal_material = Rc::new(metal_material);

        let normal_material_data = Rc::new(NormalMaterial{});
        let normal_material = Material{
            scatter : normal_material_data.clone(),
            sample: normal_material_data.clone()
        };
        let rc_normal_material = Rc::new(normal_material);

        let refraction_material_data = Rc::new(
            RefractionMaterial{refraction_type: RefractionType::Glass}
        );
        let refraction_material = Material{
            scatter : refraction_material_data.clone(),
            sample: refraction_material_data.clone()
        };
        let rc_refraction_material = Rc::new(refraction_material);

        let mut mesh : Mesh = Mesh::new(rc_diffuse_material.clone());
        mesh.add_geometry(Rc::new(Sphere{radius : 0.5, position : Vec3A::new(0.0, 0.0, 1.2)}));
        mesh.add_geometry(Rc::new(Sphere{radius : 100.0, position : Vec3A::new(0.0, -100.5, 1.0)}));
        self.meshes.push(mesh);
        
        let mut mesh : Mesh = Mesh::new(rc_metal_material.clone());
        mesh.add_geometry(Rc::new(Sphere{radius : 0.5, position : Vec3A::new(1.0, 0.0, 1.2)}));
        self.meshes.push(mesh);

        let mut mesh : Mesh = Mesh::new(rc_normal_material.clone());
        mesh.add_geometry(Rc::new(Sphere{radius : 0.5, position : Vec3A::new(-1.0, 0.0, 1.2)}));
        self.meshes.push(mesh);

        let mut mesh : Mesh = Mesh::new(rc_refraction_material.clone());
        mesh.add_geometry(Rc::new(Sphere{radius : 0.4, position : Vec3A::new(-0.5, 0.3, 0.7)}));
        self.meshes.push(mesh);

        self.load_gltf("example1.gltf");

        let mut mesh : Mesh = Mesh::new(rc_normal_material.clone());
        mesh.add_geometry(Rc::new(Triangle::DEFAULT));
        self.meshes.push(mesh);
    }
}