use crate::geometry::ray::Ray;
use crate::math::vector3::Vector3;

pub struct HitRecord {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f32, point: Vector3, normal: Vector3, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        
        Self {
            t,
            point,
            normal,
            front_face,
        }
    }
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord>;
}