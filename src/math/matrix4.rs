use super::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn translation(v: Vector3) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, v.x],
                [0.0, 1.0, 0.0, v.y],
                [0.0, 0.0, 1.0, v.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn scaling(v: Vector3) -> Self {
        Self {
            m: [
                [v.x, 0.0, 0.0, 0.0],
                [0.0, v.y, 0.0, 0.0],
                [0.0, 0.0, v.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    pub fn multiply_vector3(&self, v: Vector3) -> Vector3 {
        let x = self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3];
        let y = self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3];
        let z = self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3];
        Vector3::new(x, y, z)
    }
    
    pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
        let mut result = Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = 0.0;
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }
}

impl std::ops::Mul<Vector3> for Matrix4 {
    type Output = Vector3;
    
    fn mul(self, v: Vector3) -> Vector3 {
        self.multiply_vector3(v)
    }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    
    fn mul(self, other: Matrix4) -> Matrix4 {
        self.multiply(&other)
    }
}