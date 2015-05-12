extern crate image;

use vector::*;

pub type Color = Vec3;

pub enum Object {
    Sphere { center: Point, radius: f64 }
}

pub struct Light {
    pub position: Point,
    pub color: Color,
    pub intensity: f64
}

pub struct Camera {
    pub position: Point,
    pub direction: Vec3,
    pub up: Vec3
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl Color {
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb { data: [(self.x * 255.0) as u8,
                            (self.y * 255.0) as u8,
                            (self.z * 255.0) as u8] }
    }
}

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(&Object, f64)> {
        for object in self.objects.iter() {
            match intersects(&ray, &object) {
                Some(t) => {
                    return Some((&object, t))
                }
                None => {}
            }
        }
        None
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
