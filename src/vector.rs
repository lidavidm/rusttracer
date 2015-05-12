use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point = Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    #[inline(always)]
    pub fn dot(self, other: Vec3) -> f64 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;
        let Vec3 { x: x2, y: y2, z: z2 } = other;

        (x1 * x2) + (y1 * y2) + (z1 * z2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;
        let Vec3 { x: x2, y: y2, z: z2 } = other;

        Vec3 { x: x1 + x2, y: y1 + y2, z: z1 + z2 }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;
        let Vec3 { x: x2, y: y2, z: z2 } = other;

        Vec3 { x: x1 - x2, y: y1 - y2, z: z1 - z2 }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: f64) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;

        Vec3 { x: other*x1, y: other*y1, z: other*z1 }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;

        Vec3 { x: -x1, y: -y1, z: -z1 }
    }
}
