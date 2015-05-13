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
        self.objects.iter().fold(None, |result, materialobject| {
            let (ref object, _) = *materialobject;
            match intersects(&ray, object) {
                Some(t) => {
                    match result {
                        Some((_, t_prev)) => {
                            if t < t_prev {
                                Some((&materialobject, t))
                            }
                            else {
                                result
                            }
                        }
                        None => {
                            Some((&materialobject, t))
                        }
                    }
                }
                None => {
                    result
                }
            }
        })
    }
}

const T_THRESHOLD: f64 = 0.00000001;

pub fn intersects(ray: &Ray, object: &Object) -> Option<f64> {
    let Ray { origin: e, direction: d } = *ray;
    match *object {
        Object::Sphere { center: c, radius: r } => {
            let discrim = d.dot(e - c).powi(2) -
                (d.dot(d)) * ((e - c).dot(e-c) - r.powi(2));
            if discrim > T_THRESHOLD {
                let sqrtd = discrim.sqrt();
                let t1 = ((-d).dot(e - c) - sqrtd) / (d.dot(d));
                let t2 = ((-d).dot(e - c) + sqrtd) / (d.dot(d));
                match (t1 >= T_THRESHOLD, t2 >= T_THRESHOLD, t1 >= t2) {
                    (true, false, true) |
                    (true, true, false) |
                    (true, false, false) => {
                        Some(t1)
                    }
                    (false, true, true) |
                    (false, true, false) |
                    (true, true, true) => {
                        Some(t2)
                    }
                    _ => {
                        None
                    }
                }
            }
            else {
                None
            }
        }
    }
}
