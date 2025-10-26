use crate::math::vector3::Vector3;

pub struct Skybox {
    top_color: Vector3,
    bottom_color: Vector3,
    horizon_color: Vector3,
}

impl Skybox {
    pub fn new(top_color: Vector3, bottom_color: Vector3) -> Self {
        Self {
            top_color,
            bottom_color,
            horizon_color: (top_color + bottom_color) * 0.5,
        }
    }

    pub fn sample(&self, direction: Vector3) -> Vector3 {
        let normalized_dir = direction.normalize();
        let y = normalized_dir.y;
        
        if y >= 0.0 {
            let t = y; 
            self.horizon_color * (1.0 - t) + self.top_color * t
        } else {
            let t = -y;
            self.horizon_color * (1.0 - t) + self.bottom_color * t
        }
    }

    pub fn gradient_sky() -> Self {
        Self::new(
            Vector3::new(0.2, 0.4, 0.8),   
            
            Vector3::new(0.1, 0.1, 0.3),   
            
        )
    }

    pub fn sunset_sky() -> Self {
        Self::new(
            Vector3::new(1.0, 0.5, 0.2), 
            
            Vector3::new(0.3, 0.1, 0.4),  
            
        )
    }
}