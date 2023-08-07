use std::vec;
use glam::{Vec3A, Vec4, Mat4};
use image::GenericImageView;
use crate::engine::geometry::bvh::aabb::AABB;
use crate::engine::material::*;
use crate::engine::material::diffuse::*;
use crate::engine::material::diffuse_light::*;
use crate::engine::material::metal::*;
use crate::engine::material::normal::*;
use crate::engine::material::refraction::*;
use crate::engine::material::pbr::*;
use crate::engine::material::uv::*;
use crate::engine::texture::texture2d::*;
use crate::engine::texture::*;
use crate::engine::geometry::bvh::node::*;
use crate::engine::geometry::sphere::*;
use crate::engine::math::utils::*;
use crate::engine::camera::*;

use super::geometry::sphere;
use super::geometry::traceable::Traceable;
use super::geometry::triangle::*;
use super::geometry::vertex::Vertex;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::ColorType;

use data_url::{DataUrl};

use std::sync::{Arc};
use std::io;
use std::fs;

pub struct Scene {
    pub geometry: Vec<Arc<dyn Traceable>>,
    pub lights: Vec<Arc<dyn Traceable>>,
    pub materials: Vec<Arc<dyn Material>>,
    pub textures: Vec<Arc<Texture>>,
    pub bvh: Node,
    pub cameras: Vec<Arc<PerspectiveCamera>>
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
            lights: Vec::new(),
            materials : Vec::new(),
            textures : Vec::new(),
            cameras: Vec::new(),
        }
    }

    pub fn build_bvh(&mut self) {
        self.bvh = Node::new(&self.geometry, 0, self.geometry.len());
    }

    fn load_gltf_material(&mut self, context : &mut GLTFContext, material: gltf::material::Material) -> Arc<dyn Material> {
        let mut pbr_material = PBRMaterial::new();

        let pbr_metallic_roughness = material.pbr_metallic_roughness();
        let pbr_base_color_texture_option = pbr_metallic_roughness.base_color_texture();
        if pbr_base_color_texture_option.is_some() {
            let base_color_texture = pbr_base_color_texture_option.unwrap();
            let image = &mut context.decoded_images[base_color_texture.texture().source().index()];
            image.set_uv_index(base_color_texture.tex_coord() as usize);
            pbr_material.pbr_metallic_roughness.base_color_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let pbr_base_color_factor = pbr_metallic_roughness.base_color_factor();
        pbr_material.pbr_metallic_roughness.base_color_factor = Vec4::from(pbr_base_color_factor);

        let pbr_metalic_roughness_texture_option = pbr_metallic_roughness.metallic_roughness_texture();
        if pbr_metalic_roughness_texture_option.is_some() {
            let metalic_roughness_texture = pbr_metalic_roughness_texture_option.unwrap();
            let image = &mut context.decoded_images[metalic_roughness_texture.texture().source().index()];
            image.set_uv_index(metalic_roughness_texture.tex_coord() as usize);
            pbr_material.pbr_metallic_roughness.metalic_roughness_texture = Arc::new(Texture2D::new(image.clone()));
        }

        pbr_material.pbr_metallic_roughness.metalic_factor = pbr_metallic_roughness.metallic_factor();
        pbr_material.pbr_metallic_roughness.roughness_factor = pbr_metallic_roughness.roughness_factor();

        let normal_texture_option = material.normal_texture();
        if normal_texture_option.is_some() {
            let normal_texture = normal_texture_option.unwrap();
            let mut image = &mut context.decoded_images[normal_texture.texture().source().index()];
            image.set_uv_index(normal_texture.tex_coord() as usize);
            pbr_material.pbr_metallic_roughness.normal_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let occlusion_texture_option = material.occlusion_texture();
        if occlusion_texture_option.is_some() {
            let occlusion_texture = occlusion_texture_option.unwrap();
            let image = &mut context.decoded_images[occlusion_texture.texture().source().index()];
            image.set_uv_index(occlusion_texture.tex_coord() as usize);
            pbr_material.occlusion_texture = Arc::new(Texture2D::new(image.clone()));
        }
        let emissive_texture_option = material.emissive_texture();
        if emissive_texture_option.is_some() {
            let emissive_texture = emissive_texture_option.unwrap();
            let image = &mut context.decoded_images[emissive_texture.texture().source().index()];
            image.set_uv_index(emissive_texture.tex_coord() as usize);
            pbr_material.emissive_texture = Arc::new(Texture2D::new(image.clone()));
        }
        pbr_material.emissive_factor = Vec3A::from(material.emissive_factor());

        Arc::new(pbr_material)
    }

    fn load_gltf_node(&mut self, context : &mut GLTFContext, node: &gltf::Node, matrix: &Mat4) {
        let node_transform_matrix = node.transform().matrix();
        let new_matrix = matrix.mul_mat4(&Mat4::from_cols_array_2d(&node_transform_matrix));

        let mesh_option =  node.mesh();
        if mesh_option.is_some() {
            let gltf_mesh = mesh_option.unwrap();

            for primitive in gltf_mesh.primitives() {
                let mut positions: Vec<[Vec3A; 3]> = Vec::new();
                let mut uvs: Vec<[Vec3A; 3]> = Vec::new();
                let mut normals: Vec<[Vec3A; 3]> = Vec::new();
                let mut binormals: Vec<[Vec3A; 3]> = Vec::new();
                let mut tangents: Vec<[Vec3A; 3]> = Vec::new();

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
                                    let mut decoded_triangle = decode_triangle_vec3_indexed(
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
                                    let decoded_triangle = decode_triangle_vec2_indexed(
                                        buffer, buffer_view.offset(), stride, raw_type.size(),
                                        indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_size
                                    );

                                    uvs.push([
                                        Vec3A::from((decoded_triangle[0], set as f32)),
                                        Vec3A::from((decoded_triangle[1], set as f32)), 
                                        Vec3A::from((decoded_triangle[2], set as f32)),  
                                        ]);
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
                                        let mut decoded_triangle = decode_triangle_vec3_indexed(
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
                                        let decoded_triangle = decode_triangle_vec2_indexed(
                                            buffer, buffer_view.offset(), stride, raw_type.size(),
                                            indices_buffer, pos_raw_indices_data_pos, indices_stride, indices_raw_type.size()
                                        );

                                        uvs.push([
                                            Vec3A::from((decoded_triangle[0], set as f32)),
                                            Vec3A::from((decoded_triangle[1], set as f32)), 
                                            Vec3A::from((decoded_triangle[2], set as f32)),  
                                            ]);
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
                                        let mut decoded_triangle =  decode_triangle_vec3(
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
                                        let decoded_triangle = decode_triangle_vec2(
                                            buffer, pos_raw_data_pos, stride, raw_type.size()
                                        );

                                        uvs.push([
                                            Vec3A::from((decoded_triangle[0], set as f32)),
                                            Vec3A::from((decoded_triangle[1], set as f32)), 
                                            Vec3A::from((decoded_triangle[2], set as f32)),  
                                            ]);
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

                for (index, normal) in normals.iter().enumerate() {
                    let delta_pos1 = positions[index][1] - positions[index][0];
                    let delta_pos2 = positions[index][2] - positions[index][0];

                    let delta_uv1 = uvs[index][1] - uvs[index][0];
                    let delta_uv2 = uvs[index][2] - uvs[index][0];

                    let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);

                    let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                    let binormal = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * r; 

                    let tangent = [
                        tangent - normal[0].dot(tangent) * normal[0], 
                        tangent - normal[1].dot(tangent) * normal[1], 
                        tangent - normal[2].dot(tangent) * normal[2], 
                    ];

                    let binormal = [
                        binormal - normal[0].dot(binormal) * normal[0] - tangent[0].dot(binormal) * tangent[0], 
                        binormal - normal[1].dot(binormal) * normal[1] - tangent[1].dot(binormal) * tangent[1], 
                        binormal - normal[2].dot(binormal) * normal[2] - tangent[2].dot(binormal) * tangent[2], 
                    ];

                    tangents.push(tangent);
                    binormals.push(binormal);
                }

                assert!(positions.len() == 0 || positions.len() == normals.len());
                assert!(normals.len() == 0||  uvs.len() % normals.len() == 0);
                let triangles_count = positions.len();

                let mut mesh_triangles: Vec<Arc<Box<dyn Traceable>>> = Vec::new();
                mesh_triangles.reserve(triangles_count);

                for i in 0..triangles_count {
                    let uvs_count = uvs.len() / triangles_count;

                    let mut uvs1 = Vec::new();
                    let mut uvs2 = Vec::new();
                    let mut uvs3 = Vec::new();
                    for j in 0..uvs_count {
                        uvs1.push(uvs[j * triangles_count + i][0]);
                        uvs2.push(uvs[j * triangles_count + i][1]);
                        uvs3.push(uvs[j * triangles_count + i][2]);
                    }

                    let vertex1 = Vertex::new(positions[i][0], normals[i][0], 
                        binormals[i][0], tangents[i][0], uvs1);
                    let vertex2 = Vertex::new(positions[i][1], normals[i][1], 
                        binormals[i][1], tangents[i][1], uvs2);
                    let vertex3 = Vertex::new(positions[i][2], normals[i][2], 
                        binormals[i][2], tangents[i][2], uvs3);

                    self.geometry.push(Arc::new(Triangle::new(material.clone(), vertex1, vertex2, vertex3)));
                }

                self.materials.push(material);
            }
        }

        let camera_option = node.camera();
        if camera_option.is_some() {
            let camera = camera_option.unwrap();
            match camera.projection() {
                gltf::camera::Projection::Orthographic(orthographic) => {
                    /*self.cameras.push(Arc::new(
                        OrthographicCamera::new()
                    ))*/
                },
                gltf::camera::Projection::Perspective(perspective) => {
                    let mut aspect_ratio = 1.0;
                    if perspective.aspect_ratio().is_some() {
                        aspect_ratio = perspective.aspect_ratio().unwrap();
                    }
                    let mut z_far = perspective.znear();
                    if perspective.zfar().is_some() {
                        z_far = perspective.zfar().unwrap();
                    }

                    let mut name = "Default";
                    if camera.name().is_some() {
                        name = camera.name().unwrap();
                    }

                    self.cameras.push(Arc::new(
                        PerspectiveCamera::new(
                            &new_matrix,
                            perspective.yfov(),
                            aspect_ratio,
                            perspective.znear(),
                            z_far,
                            name
                        )
                    ))
                },
            }
        }

        for child in node.children() {
            self.load_gltf_node(context, &child, &new_matrix);
        }
    }

    pub fn load_gltf(&mut self, path: &str) {
        let file = fs::File::open(path).expect(format!("Invalid filename: {}", path).as_str());
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

        context.decoded_images.resize(gltf.images().count(), Texture::null());
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
                            let (bytes_per_component, components_per_pixel) = match value.color() {
                                ColorType::L8 => (1, 1),
                                ColorType::La8 => (1, 2),
                                ColorType::Rgb8 => (1, 3),
                                ColorType::Rgba8 => (1, 4),
                                ColorType::Bgra8 => (1, 4),
                                ColorType::Bgr8 => (1, 3),
                                ColorType::L16 => (2, 1),
                                ColorType::La16 => (2, 2),
                                ColorType::Rgb16 => (2, 3),
                                ColorType::Rgba16 => (2, 4),
                                _ => (0, 0)
                            };

                            context.decoded_images[image.index()] = Texture::new(
                                vec![value.width(), value.height()],
                                bytes_per_component, components_per_pixel,
                                value
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
                self.load_gltf_node(&mut context, &node, &Mat4::IDENTITY);
            }
        }
    }

    pub fn load_debug(&mut self) {
        let diffuse_material = Arc::new(DiffuseMaterial{});
        let metal_material = Arc::new(
            MetalMaterial{metalness : 0.9}
        );
        let normal_material = Arc::new(NormalMaterial{diffuse: DiffuseMaterial{}});
        let refraction_material = Arc::new(
            RefractionMaterial{refraction_type: RefractionType::Glass}
        );
        let uv_material = Arc::new(
            UVMaterial{diffuse: DiffuseMaterial{}}
        );
        let diffuse_light_material1 = Arc::new(
            DiffuseLightMaterial{color: Vec3A::new(1.4, 0.1, 0.2)}
        );
        let diffuse_light_material2 = Arc::new(
            DiffuseLightMaterial{color: Vec3A::new(0.05, 1.3, 0.0)}
        );
        let diffuse_light_material3 = Arc::new(
            DiffuseLightMaterial{color: Vec3A::new(1.2, 1.2, 1.5)}
        );
        let diffuse_light_material4 = Arc::new(
            DiffuseLightMaterial{color: Vec3A::new(0.1, 0.05, 1.5)}
        );

        let sphere1 = Arc::new(Sphere::new(diffuse_light_material1.clone(), 0.2, Vec3A::new(-6.5, 0.5, -1.5)));
        let sphere2 = Arc::new(Sphere::new(diffuse_light_material2.clone(), 0.2, Vec3A::new(-2.5, 2.0, 0.0)));
        let sphere3 = Arc::new(Sphere::new(diffuse_light_material3.clone(), 0.2, Vec3A::new(-6.5, 0.5, 1.0)));
        let sphere4 = Arc::new(Sphere::new(diffuse_light_material4.clone(), 0.2, Vec3A::new(8.0, 1.5, -0.5)));

        self.lights.push(sphere1.clone());
        self.geometry.push(sphere1.clone());
        self.lights.push(sphere2.clone());
        self.geometry.push(sphere2.clone());
        self.lights.push(sphere3.clone());
        self.geometry.push(sphere3.clone());
        self.lights.push(sphere4.clone());
        self.geometry.push(sphere4.clone());
        //self.geometry.push(Arc::new(Sphere{material: diffuse_material.clone(), radius : 100.0, position : Vec3A::new(0.0, -101.0, 1.0)}));
        //self.geometry.push(Arc::new(Sphere{material: metal_material.clone(), radius : 0.5, position : Vec3A::new(1.0, 0.0, 1.2)}));
        //self.geometry.push(Arc::new(Sphere{material: normal_material.clone(), radius : 0.5, position : Vec3A::new(-1.0, 0.0, 1.2)}));
        //self.geometry.push(Arc::new(Sphere{material: refraction_material.clone(), radius : 0.5, position : Vec3A::new(-1.3, 0.15, 0.5)}));

        self.materials.push(diffuse_material.clone());
        self.materials.push(metal_material.clone());
        self.materials.push(normal_material.clone());
        self.materials.push(refraction_material.clone());
        self.materials.push(uv_material.clone());
        self.materials.push(diffuse_light_material1.clone());
        self.materials.push(diffuse_light_material2.clone());
        self.materials.push(diffuse_light_material3.clone());
        self.materials.push(diffuse_light_material4.clone());
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}