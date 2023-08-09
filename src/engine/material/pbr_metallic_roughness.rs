use crate::engine::{material::*, onb::ONB};

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::math::utils::*;
use crate::engine::texture::texture2d::*;
use crate::engine::sampler::sampler::*;
use crate::engine::material::pdf::cook_torrance_distribution::*;

use glam::{Vec2, Vec3A, Vec4};

use super::pdf::PDFBase;

pub struct PBRMetallicRoughnessMaterial {
    pub base_color_factor: Vec4,

    pub base_color_texture: Arc<Texture2D>,
    pub base_color_texture_sampler: Sampler,

    pub normal_texture: Arc<Texture2D>,
    pub normal_texture_sampler: Sampler,

    pub metalic_roughness_texture: Arc<Texture2D>,
    pub metalic_roughness_texture_sampler: Sampler,
    pub metalic_factor: f32,
    pub roughness_factor: f32,
}

impl PBRMetallicRoughnessMaterial {
    fn ggx_partial_geometry(cos_theta_n: f32, alpha : f32) -> f32 {
        let cos_theta_sqr = cos_theta_n * cos_theta_n;
        let tan2 = ( 1.0 - cos_theta_sqr ) / cos_theta_sqr;
        let GP = 2.0 / ( 1.0 + ( 1.0 + alpha * alpha * tan2 ).sqrt() );
        return GP;
    }

    fn ggx_distribution(cos_theta_NH: f32, alpha: f32) -> f32 {
        let alpha2 = alpha * alpha;
        let NH_sqr = cos_theta_NH * cos_theta_NH;
        let den = NH_sqr * alpha2 + (1.0 - NH_sqr);
        return alpha2 / ( std::f32::consts::PI * den * den );
    }

    fn fresnel_schlick(F0: Vec3A, cos_theta: f32) -> Vec3A {
        return F0 + (1.0 - F0) * (1.0 - cos_theta).powf(5.0);
    }

    fn cook_torrance_ggx_specular(&self, normal: Vec3A, light: Vec3A, view: Vec3A, 
        albedo: Vec3A, roughness: f32, metallic: f32) -> (Vec3A, Vec3A, f32) {       
        let light = light.normalize();

        let roughness_sqr = roughness * roughness;

        let H = (view + light).normalize();

        let normal_light_cos = normal.dot(light);
        if normal_light_cos <= 0.0 {
            return (Vec3A::ZERO, Vec3A::ZERO, 1.0);
        }
        let normal_view_cos = normal.dot(view);
        if normal_view_cos <= 0.0 {
            return (Vec3A::ZERO, Vec3A::ZERO, 1.0);
        }
        let normal_h_cos = normal.dot(H);
        let h_view_cos = H.dot(view);

        let mut f0 = Vec3A::new(0.04, 0.04, 0.04);
        f0 = (1.0 - metallic) * f0 + metallic * Vec3A::from(albedo);

        let G = Self::ggx_partial_geometry(normal_view_cos, roughness_sqr) *
            Self::ggx_partial_geometry(normal_light_cos, roughness_sqr);
        let D = Self::ggx_distribution(normal_h_cos, roughness_sqr);
        let F = Self::fresnel_schlick(f0, h_view_cos);
    
        let pdf = D * normal_h_cos / (4.0 * h_view_cos); //и вычисление самой pdf

        let spec_k = G * F / (normal_view_cos * normal_h_cos);

        return (F, spec_k.max(Vec3A::ZERO), pdf);
    }

    pub fn new() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            base_color_texture: Arc::new(Texture2D::null()),
            base_color_texture_sampler: Sampler::new(),
            metalic_roughness_texture:  Arc::new(Texture2D::null()),
            metalic_roughness_texture_sampler: Sampler::new(),
            normal_texture:  Arc::new(Texture2D::null()),
            normal_texture_sampler: Sampler::new(),
            metalic_factor: 0.0,
            roughness_factor: 0.0
        }
    }
}

pub fn reflect(eye: Vec3A, normal: Vec3A) -> Vec3A {
    eye - 2.0 * (normal.dot(eye)) * normal
}

impl Material for PBRMetallicRoughnessMaterial {
    fn scatter(&self, ray: &Ray, hit_result : &HitResult) -> ScatterResult {
        let mut albedo = Vec4::ONE;
        albedo *= self.base_color_factor;
        albedo = albedo * self.base_color_texture.sample(
            &self.base_color_texture_sampler, 
            self.base_color_texture.texture.get_uv_by_index(&hit_result.uvs)
        );

        let mut normal = hit_result.normal;

        if self.normal_texture.valid() {
            let mut normal_map = Vec3A::from(self.normal_texture.sample(
                &self.normal_texture_sampler, 
                self.normal_texture.texture.get_uv_by_index(&hit_result.uvs)
            ));
            normal_map = normal_map * 2.0 - Vec3A::ONE;

            /*normal = normal + 
                hit_result.tangent * normal_map.x + 
                hit_result.binormal * normal_map.y;

            normal = normal.normalize();*/
        }  

        let metallic_roughness = self.metalic_roughness_texture.sample(
            &self.metalic_roughness_texture_sampler, 
            self.metalic_roughness_texture.texture.get_uv_by_index(&hit_result.uvs)
        );

        let roughness = metallic_roughness.y * self.roughness_factor;
        let metallic = metallic_roughness.x * self.metalic_factor;

        let mut scatter_pdf = CookTorranceDistributionPDF::new(normal);

        scatter_pdf.generated_direction = scatter_pdf.base_pdf.basis.get_position(
            random_ggx_hemisphere_direction(roughness * roughness)
        ).normalize();

        let light = scatter_pdf.generated_direction;

        let (fresnel, specular, pdf) = self.cook_torrance_ggx_specular(normal, 
            light, -ray.direction, Vec3A::from(albedo),
            roughness, metallic);

        scatter_pdf.pdf = pdf;

        let mut sample = Vec3A::from(albedo) * (1.0 - fresnel) + specular;

        let mut scatter_result = ScatterResult{
            attenuation: Vec3A::ONE, 
            scatter: Some(Rc::new(scatter_pdf)),
            alpha_masked: false,
            hit_result: hit_result.clone()
        };

        if albedo.w < 0.99 {
            sample = Vec3A::ONE;
            scatter_result.alpha_masked = true;
        }

        scatter_result.attenuation = sample;

        return scatter_result;
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}