use crate::math::vector3::Vector3;
use super::texture::Texture;

#[derive(Clone, Debug)]
pub struct Material {
    pub albedo: Vector3,       
    pub albedo_texture: Option<Texture>, 
    pub specular: f32,         
    pub shininess: f32,      
    pub reflectivity: f32,      
    pub transparency: f32,      
    pub refractive_index: f32,  
    pub emissive: Vector3,      
    pub emissive_intensity: f32,
    pub normal_map: Option<Texture>, 
}

impl Material {
    pub fn new(
        albedo: Vector3,
        specular: f32,
        shininess: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            albedo,
            albedo_texture: None,
            specular,
            shininess,
            reflectivity: reflectivity.clamp(0.0, 1.0),
            transparency: transparency.clamp(0.0, 1.0),
            refractive_index,
            emissive: Vector3::zero(),
            emissive_intensity: 0.0,
            normal_map: None,
        }
    }

    pub fn new_emissive(
        albedo: Vector3,
        specular: f32,
        shininess: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
        emissive: Vector3,
        emissive_intensity: f32,
    ) -> Self {
        Self {
            albedo,
            albedo_texture: None,
            specular,
            shininess,
            reflectivity: reflectivity.clamp(0.0, 1.0),
            transparency: transparency.clamp(0.0, 1.0),
            refractive_index,
            emissive,
            emissive_intensity,
            normal_map: None,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.0
    }

    pub fn is_reflective(&self) -> bool {
        self.reflectivity > 0.0
    }

    pub fn is_emissive(&self) -> bool {
        self.emissive_intensity > 0.0
    }

    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.albedo_texture = Some(texture);
        self
    }

    pub fn with_normal_map(mut self, texture: Texture) -> Self {
        self.normal_map = Some(texture);
        self
    }
}

impl Material {
    pub fn wood() -> Self {
        Self::new(
            Vector3::new(0.6, 0.4, 0.2), 
            0.3, 32.0, 0.1, 0.0, 1.0
        )
    }

    pub fn metal() -> Self {
        Self::new(
            Vector3::new(0.8, 0.8, 0.9), 
            0.9, 128.0, 0.7, 0.0, 1.0
        )
    }

    pub fn glass() -> Self {
        Self::new(
            Vector3::new(0.9, 0.9, 1.0), 
            0.8, 256.0, 0.3, 0.8, 1.5
        )
    }

    pub fn stone() -> Self {
        Self::new(
            Vector3::new(0.5, 0.5, 0.5), 
            0.2, 16.0, 0.05, 0.0, 1.0
        )
    }

    pub fn plastic() -> Self {
        Self::new(
            Vector3::new(0.2, 0.6, 0.2), 
            0.6, 64.0, 0.4, 0.0, 1.0
        )
    }
}