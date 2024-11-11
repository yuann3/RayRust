use core::fmt;
use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    // Getters
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    // Vector length
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    // Utility methods
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + 
        self.e[1] * other.e[1] + 
        self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

/// Operator Over loading
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign (&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 { 
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t], 
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x(), 0.0);
        assert_eq!(v3.y(), 0.0);
        assert_eq!(v3.z(), 1.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(3.0, 0.0, 0.0);
        let u = v.unit_vector();
        assert_eq!(u.x(), 1.0);
        assert_eq!(u.y(), 0.0);
        assert_eq!(u.z(), 0.0);
    }
}
