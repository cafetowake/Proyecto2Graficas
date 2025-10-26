use crate::math::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}