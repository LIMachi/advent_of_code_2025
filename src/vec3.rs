use std::ops::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn dist_squared(&self, rhs: &Vec3) -> f64 {
        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;
        let dz = self.z as f64 - rhs.z as f64;
        dx * dx + dy * dy + dz * dz
    }
    
    pub fn dist(&self, rhs: &Vec3) -> f64 {
        self.dist_squared(&rhs).sqrt()
    }

    pub fn scale(&self, mut scalar: i64) -> Self {
        if scalar >= 0 {
            Self {
                x: self.x * scalar,
                y: self.y * scalar,
                z: self.z * scalar,
            }
        } else {
            scalar = -scalar;
            Self {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<i64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}