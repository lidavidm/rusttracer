extern crate image;

use vector::*;

pub type Color = Vec3;

pub enum Object {
    Sphere { center: Point, radius: f64 }
}

pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub specular_exponent: f64
}

pub type MaterialObject = (Object, Material);

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
    pub objects: Vec<MaterialObject>,
}

impl Color {
    pub fn to_rgb(&self) -> image::Rgb<u8> {
        // TODO: implement tonemapping (by parameterizing color on a trait?)
        image::Rgb { data: [(self.x.min(1.0) * 255.0) as u8,
                            (self.y.min(1.0) * 255.0) as u8,
                            (self.z.min(1.0) * 255.0) as u8] }
    }
}

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(&MaterialObject, f64)> {
        for materialobject in self.objects.iter() {
            let (ref object, _) = *materialobject;
            match intersects(&ray, object) {
                Some(t) => {
                    return Some((&materialobject, t))
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
