#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }
    
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
    
    pub fn refract(self, normal: Self, eta: f32) -> Option<Self> {
        let dot = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - dot * dot);
        
        if k < 0.0 {
            None
        } else {
            Some((self - normal * dot) * eta - normal * k.sqrt())
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    
    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}