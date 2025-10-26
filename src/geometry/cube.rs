use crate::math::vector3::Vector3;
use crate::geometry::ray::Ray;
use crate::materials::material::Material;

pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, material: Material) -> Self {
        Self { min, max, material }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Vector3, (f32, f32))> {
        let inv_direction = Vector3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t1 = (self.min - ray.origin) * inv_direction;
        let t2 = (self.max - ray.origin) * inv_direction;

        let t_min = Vector3::new(
            t1.x.min(t2.x),
            t1.y.min(t2.y),
            t1.z.min(t2.z),
        );
        let t_max = Vector3::new(
            t1.x.max(t2.x),
            t1.y.max(t2.y),
            t1.z.max(t2.z),
        );

        let t_enter = t_min.x.max(t_min.y).max(t_min.z);
        let t_exit = t_max.x.min(t_max.y).min(t_max.z);

        if t_enter < t_exit && t_exit > 0.0 {
            let t_hit = if t_enter > 0.0 { t_enter } else { t_exit };
            let point = ray.at(t_hit);
            let normal = self.calculate_normal(point);
            let (u, v) = self.compute_uv(point, normal);
            Some((t_hit, normal, (u, v)))
        } else {
            None
        }
    }

    fn calculate_normal(&self, point: Vector3) -> Vector3 {
        let epsilon = 0.0001;
        
        if (point.x - self.min.x).abs() < epsilon {
            Vector3::new(-1.0, 0.0, 0.0) 
        } else if (point.x - self.max.x).abs() < epsilon {
            Vector3::new(1.0, 0.0, 0.0)  
        } else if (point.y - self.min.y).abs() < epsilon {
            Vector3::new(0.0, -1.0, 0.0) 
        } else if (point.y - self.max.y).abs() < epsilon {
            Vector3::new(0.0, 1.0, 0.0)  
        } else if (point.z - self.min.z).abs() < epsilon {
            Vector3::new(0.0, 0.0, -1.0) 
        } else {
            Vector3::new(0.0, 0.0, 1.0) 
        }
    }

    fn compute_uv(&self, point: Vector3, normal: Vector3) -> (f32, f32) {
        let eps = 1e-5;
        let dx = (self.max.x - self.min.x).max(eps);
        let dy = (self.max.y - self.min.y).max(eps);
        let dz = (self.max.z - self.min.z).max(eps);

        if normal.x.abs() > 0.5 {
            let u = (point.z - self.min.z) / dz;
            let v = (point.y - self.min.y) / dy;
            (u.clamp(0.0,1.0), v.clamp(0.0,1.0))
        } else if normal.y.abs() > 0.5 {
            let u = (point.x - self.min.x) / dx;
            let v = (point.z - self.min.z) / dz;
            (u.clamp(0.0,1.0), v.clamp(0.0,1.0))
        } else {
            let u = (point.x - self.min.x) / dx;
            let v = (point.y - self.min.y) / dy;
            (u.clamp(0.0,1.0), v.clamp(0.0,1.0))
        }
    }

    pub fn get_center(&self) -> Vector3 {
        (self.min + self.max) * 0.5
    }
}