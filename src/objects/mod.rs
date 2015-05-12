extern crate image;

use vector::*;

pub type Color = Vec3;

pub enum Object {
    Sphere { center: Point, radius: f64 }
}

pub struct Light {
    position: Point,
    color: Color,
    intensity: f64
}

impl Color {
    fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb { data: [(self.x * 255.0) as u8,
                            (self.y * 255.0) as u8,
                            (self.z * 255.0) as u8] }
    }
}

const T_THRESHOLD: f64 = 0.00001;

#[inline(always)]
fn square(x: f64) -> f64 {
    x * x
}

pub fn intersects(ray: &Ray, object: &Object) -> Option<f64> {
    let Ray { origin: e, direction: d } = *ray;
    match *object {
        Object::Sphere { center: c, radius: r } => {
            let discrim = square(d.dot(e - c)) -
                (d.dot(d)) * ((e - c).dot(e-c) - square(r));
            if discrim > T_THRESHOLD {
                Some (((-d).dot(e - c) + discrim.sqrt()) / (d.dot(d)))
            }
            else {
                None
            }
        }
    }
}
