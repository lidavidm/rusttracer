use std::ops::{Add, Sub, Mul, Neg, Div};

/// A 3-dimensional vector.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// A point in R^3 (alias of a vector).
pub type Point = Vec3;

/// A ray in R^3.
#[derive(Debug)]
pub struct Ray {
    /// The starting point of the ray.
    pub origin: Vec3,
    /// The direction the ray travels. Not necessarily normalized.
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

    pub fn cross(self, other: Vec3) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;
        let Vec3 { x: x2, y: y2, z: z2 } = other;

        Vec3 { x: y1 * z2 - z1 * y2,
               y: z1 * x2 - x1 * z2,
               z: x1 * y2 - y1 * x2 }
    }

    pub fn mag_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn normalized(self) -> Vec3 {
        let mag = self.mag_squared().sqrt();
        let Vec3 { x: x1, y: y1, z: z1 } = self;

        Vec3 { x: x1 / mag, y: y1 / mag, z: z1 / mag }
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;
        let Vec3 { x: x2, y: y2, z: z2 } = other;

        Vec3 { x: x1 * x2, y: y1 * y2, z: z1 * z2 }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, other: f64) -> Vec3 {
        let Vec3 { x: x1, y: y1, z: z1 } = self;

        Vec3 { x: x1 / other, y: y1 / other, z: z1 / other }
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
