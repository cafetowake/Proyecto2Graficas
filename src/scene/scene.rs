use crate::geometry::cube::Cube;
use crate::geometry::ray::Ray;
use crate::lighting::phong::Light;
use crate::math::vector3::Vector3;
use crate::materials::material::Material;
use crate::textures::skybox::Skybox;
use crate::lighting::phong::PhongIllumination;

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub skybox: Option<Skybox>,
    pub ambient_color: Vector3,
    pub ambient_intensity: f32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            lights: Vec::new(),
            skybox: None,
            ambient_color: Vector3::new(0.1, 0.1, 0.1),
            ambient_intensity: 0.1,
        }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = Some(skybox);
    }

    pub fn trace_ray(&self, ray: &Ray, max_depth: u32) -> Vector3 {
        if max_depth == 0 {
            return Vector3::zero();
        }

        if let Some(hit) = self.intersect_scene(ray) {
            let material = hit.material.clone();
            let mut color = self.calculate_local_color(&hit, ray);


            if material.is_reflective() && max_depth > 1 {
                let reflection_direction = ray.direction.reflect(hit.normal);
                let reflection_ray = Ray::new(hit.point + hit.normal * 0.001, reflection_direction);
                let reflection_color = self.trace_ray(&reflection_ray, max_depth - 1);
                color = color * (1.0 - material.reflectivity) + reflection_color * material.reflectivity;
            }


            if material.is_transparent() && max_depth > 1 {
                let refraction_result = ray.direction.refract(hit.normal, material.refractive_index);
                if let Some(refraction_direction) = refraction_result {
                    let refraction_ray = Ray::new(hit.point - hit.normal * 0.001, refraction_direction);
                    let refraction_color = self.trace_ray(&refraction_ray, max_depth - 1);
                    color = color * (1.0 - material.transparency) + refraction_color * material.transparency;
                }
            }

            color
        } else {

            self.skybox.as_ref()
                .map(|skybox| skybox.sample(ray.direction))
                .unwrap_or(Vector3::new(0.2, 0.4, 0.8)) 
        }
    }

    fn intersect_scene(&self, ray: &Ray) -> Option<SceneHit> {
        let mut closest_hit: Option<SceneHit> = None;
        let mut closest_distance = f32::MAX;

        for cube in &self.cubes {
            if let Some((t, normal, uv)) = cube.intersect(ray) {
                if t < closest_distance && t > 0.001 {
                    closest_distance = t;
                    let point = ray.at(t);
                    closest_hit = Some(SceneHit {
                        point,
                        normal,
                        distance: t,
                        material: cube.material.clone(),
                        uv,
                    });
                }
            }
        }

        closest_hit
    }

    pub fn debug_intersect(&self, ray: &Ray) -> Option<(f32, (f32, f32))> {
        self.intersect_scene(ray).map(|h| (h.distance, h.uv))
    }

    fn calculate_local_color(&self, hit: &SceneHit, ray: &Ray) -> Vector3 {
        let view_direction = -ray.direction;
        PhongIllumination::calculate(
            hit.point,
            hit.normal,
            view_direction,
            &hit.material,
            hit.uv,
            &self.lights,
            self.ambient_color,
            self.ambient_intensity,
        )
    }
}

#[derive(Clone)]
struct SceneHit {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub material: Material,
    pub uv: (f32, f32),
}