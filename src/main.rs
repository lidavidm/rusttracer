mod vector;
mod objects;
mod raytrace;

use vector::*;
use objects::*;
use raytrace::*;

fn main() {
    let sphere = Object::Sphere {
        center: Vec3::new(0.0, 0.0, -2.0),
        radius: 0.5
    };
    let material = Material {
        ambient: Color::new(0.0, 0.2, 0.2),
        diffuse: Color::new(1.0, 1.0, 1.0),
        specular: Color::new(1.0, 1.0, 1.0),
        specular_exponent: 10.0
    };

    let sphere2 = Object::Sphere {
        center: Vec3::new(0.2, 0.0, -1.0),
        radius: 0.1
    };
    let material2 = Material {
        ambient: Color::new(0.2, 0.0, 0.2),
        diffuse: Color::new(1.0, 1.0, 1.0),
        specular: Color::new(1.0, 1.0, 1.0),
        specular_exponent: 100.0
    };

    let scene = Scene {
        camera: Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0)
        },
        lights: vec![
            Light {
                position: Point::new(0.5, 0.0, -0.2),
                color: Color::new(1.0, 1.0, 1.0),
                intensity: 0.6
            },
            Light {
                position: Point::new(-0.8, 0.5, -0.1),
                color: Color::new(1.0, 1.0, 1.0),
                intensity: 0.2
            }],
        objects: vec![(sphere, material), (sphere2, material2)]
    };
    raytrace(&scene, 1920, 1080, std::f64::consts::PI / 3.5);
}
