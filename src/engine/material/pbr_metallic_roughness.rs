use crate::engine::material::*;

use crate::engine::geometry::traceable::*;
use crate::engine::math::ray::*;
use crate::engine::texture::texture2d::*;
use crate::engine::sampler::sampler::*;
use super::diffuse::*;

use glam::{Vec2, Vec3A, Vec4};

pub struct PBRMetallicRoughnessMaterial {
    pub diffuse: DiffuseMaterial,

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
    fn GGX_PartialGeometry(cos_theta_n: f32, alpha : f32) -> f32 {
        let cos_theta_sqr = cos_theta_n * cos_theta_n;
        let tan2 = ( 1.0 - cos_theta_sqr ) / cos_theta_sqr;
        let GP = 2.0 / ( 1.0 + ( 1.0 + alpha * alpha * tan2 ).sqrt() );
        return GP;
    }

    fn GGX_Distribution(cos_theta_NH: f32, alpha: f32) -> f32 {
        let alpha2 = alpha * alpha;
        let NH_sqr = cos_theta_NH * cos_theta_NH;
        let den = NH_sqr * alpha2 + (1.0 - NH_sqr);
        return alpha2 / ( std::f32::consts::PI * den * den );
    }

    fn FresnelSchlick(F0: Vec3A, cos_theta: f32) -> Vec3A {
        return F0 + (1.0 - F0) * (1.0 - cos_theta).powf(5.0);
    }

    fn CookTorrance_GGX(&self, normal: Vec3A, light: Vec3A, view: Vec3A, albedo: Vec3A, hit_result : &HitResult) -> Vec4 {       
        let metallic_roughness = self.metalic_roughness_texture.sample(
            &self.metalic_roughness_texture_sampler, 
            self.metalic_roughness_texture.texture.get_uv_by_index(&hit_result.uvs)
        );

        let light = light.normalize();
        let roughness = metallic_roughness.y * self.roughness_factor;
        let roughness_sqr = roughness * roughness;
        //let roughness_sqr = 0.05;

        let H = (view + light).normalize();

        let normal_light_cos = normal.dot(light);
        if normal_light_cos <= 0.0 {
            return Vec4::ZERO;
        }
        let normal_view_cos = normal.dot(view);
        if normal_view_cos <= 0.0 {
            return Vec4::ZERO;
        }
        let normal_h_cos = normal.dot(H);
        let h_view_cos = H.dot(view);

        let mut f0 = Vec3A::new(0.04, 0.04, 0.04);
        f0 = (1.0 - metallic_roughness.x) * f0 + metallic_roughness.x * Vec3A::from(albedo);

        let G = Self::GGX_PartialGeometry(normal_view_cos, roughness_sqr) *
            Self::GGX_PartialGeometry(normal_light_cos, roughness_sqr);
        let D = Self::GGX_Distribution(normal_h_cos, roughness_sqr);
        let F = Self::FresnelSchlick(f0, h_view_cos);
    
        //mix
        let spec_k = G * D * F * 0.25 / normal_view_cos;
        let diff_k = 1.0 - F;

        let final_result = (Vec3A::from(albedo) * diff_k * normal_light_cos / std::f32::consts::PI + spec_k).max(Vec3A::ZERO);

        return Vec4::from((final_result, 0.0));
    }

    pub fn new() -> Self {
        Self {
            diffuse: DiffuseMaterial{},
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
        let mut scatter_result = self.diffuse.scatter(&ray, hit_result);

        if self.normal_texture.valid() {
            let mut normal_map = Vec3A::from(self.normal_texture.sample(
                &self.normal_texture_sampler, 
                self.normal_texture.texture.get_uv_by_index(&scatter_result.hit_result.uvs)
            ));
            normal_map = normal_map * 2.0 - Vec3A::ONE;

            scatter_result.hit_result.normal = scatter_result.hit_result.normal + 
                scatter_result.hit_result.tangent * normal_map.x + 
                scatter_result.hit_result.binormal * normal_map.y;

            scatter_result.hit_result.normal = scatter_result.hit_result.normal.normalize();
        }  

        let light_vector = Vec3A::new(0.0, 0.0, 1.0);

        let mut albedo = Vec4::ONE;
        albedo *= self.base_color_factor;
        albedo = albedo * self.base_color_texture.sample(
            &self.base_color_texture_sampler, 
            self.base_color_texture.texture.get_uv_by_index(&scatter_result.hit_result.uvs)
        );   

        let mut sample = self.CookTorrance_GGX(scatter_result.hit_result.normal, light_vector, -ray.direction, Vec3A::from(albedo),  hit_result);           

        sample = albedo;

        if albedo.w < 0.99 {
            sample = Vec4::ONE;
            scatter_result.alpha_masked = true;
        }

        scatter_result.attenuation = Vec3A::from(sample);

        return scatter_result;
    }

    fn scattering_pdf(&self, ray: &Ray, hit_result : &HitResult, scattering: &Ray) -> f32 {
        self.diffuse.scattering_pdf(&ray, hit_result, &scattering)
    }

    fn emit(&self, ray: &Ray, hit_result : &HitResult) -> Vec3A {
        Vec3A::ZERO
    }
}