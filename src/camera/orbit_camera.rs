use crate::math::vector3::Vector3;
use crate::geometry::ray::Ray;

pub struct OrbitCamera {
    center: Vector3,
    radius: f32,
    yaw: f32,      
    pitch: f32,   
    fov: f32,     
    aspect_ratio: f32,
}

impl OrbitCamera {
    pub fn new(center: Vector3, radius: f32, fov: f32, aspect_ratio: f32) -> Self {
        Self {
            center,
            radius: radius.max(1.0),
            yaw: 0.0,
            pitch: 0.0,
            fov,
            aspect_ratio,
        }
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;
        
        self.pitch = self.pitch.clamp(
            -std::f32::consts::PI / 2.0 + 0.1,
            std::f32::consts::PI / 2.0 - 0.1
        );
    }

    pub fn zoom(&mut self, delta_radius: f32) {
        self.radius = (self.radius + delta_radius).clamp(1.0, 50.0);
    }

    pub fn get_position(&self) -> Vector3 {
        let x = self.center.x + self.radius * self.yaw.cos() * self.pitch.cos();
        let y = self.center.y + self.radius * self.pitch.sin();
        let z = self.center.z + self.radius * self.yaw.sin() * self.pitch.cos();
        
        Vector3::new(x, y, z)
    }

    pub fn generate_ray(&self, x: u32, y: u32, width: u32, height: u32) -> Ray {
        let sensor_x = (2.0 * (x as f32) / (width as f32) - 1.0) * (self.fov / 2.0).tan() * self.aspect_ratio;
        let sensor_y = (1.0 - 2.0 * (y as f32) / (height as f32)) * (self.fov / 2.0).tan();

    let camera_direction = Vector3::new(sensor_x, sensor_y, 1.0).normalize();

        let camera_position = self.get_position();
        
        let forward = (self.center - camera_position).normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(forward).normalize();
        let up = forward.cross(right);

        let world_direction = 
            right * camera_direction.x + 
            up * camera_direction.y + 
            forward * camera_direction.z;

        Ray::new(camera_position, world_direction.normalize())
    }

    pub fn look_at(&mut self, target: Vector3) {
        self.center = target;
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius.clamp(1.0, 50.0);
    }

    pub fn get_center(&self) -> Vector3 {
        self.center
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}