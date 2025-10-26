use crate::math::vector3::Vector3;
use crate::materials::material::Material;

#[derive(Clone, Debug)]
pub struct Light {
    pub position: Vector3,
    pub color: Vector3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector3, color: Vector3, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}

pub struct PhongIllumination {}

impl PhongIllumination {
    pub fn calculate(
        point: Vector3,
        normal: Vector3,
        view_direction: Vector3,
        material: &Material,
        uv: (f32, f32),
        lights: &[Light],
        ambient_color: Vector3,
        ambient_intensity: f32,
    ) -> Vector3 {
        let mut result_color = Vector3::zero();

        let albedo = if let Some(tex) = &material.albedo_texture {
            tex.sample(uv.0, uv.1)
        } else {
            material.albedo
        };

        let ambient = albedo * ambient_color * ambient_intensity;
        result_color = result_color + ambient;

        if material.is_emissive() {
            let emissive = material.emissive * material.emissive_intensity;
            result_color = result_color + emissive;
        }

        for light in lights {
            let light_direction = (light.position - point).normalize();
            let distance = (light.position - point).length();
            
            let attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
            
            let diffuse_intensity = light_direction.dot(normal).max(0.0);
            let diffuse = albedo * light.color * diffuse_intensity * attenuation * light.intensity;
            
            let reflect_direction = (-light_direction).reflect(normal);
            let specular_intensity = reflect_direction.dot(view_direction).max(0.0).powf(material.shininess);
            let specular = light.color * material.specular * specular_intensity * attenuation * light.intensity;
            
            result_color = result_color + diffuse + specular;
        }

        result_color.clamp_components(0.0, 1.0)
    }
}

pub trait Vector3Ext {
    fn clamp_components(self, min: f32, max: f32) -> Self;
}

impl Vector3Ext for Vector3 {
    fn clamp_components(self, min: f32, max: f32) -> Self {
        Vector3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }
}