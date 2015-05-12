mod vector;
mod objects;
mod raytrace;

use vector::*;
use objects::*;
use raytrace::*;

fn main() {
    let sphere = Object::Sphere {
        center: Vec3::new(0.0, 0.0, -10.0),
        radius: 2.0
    };
    let material = Material {
        ambient: Color::new(0.0, 0.2, 0.2),
        diffuse: Color::new(1.0, 1.0, 1.0)
    };

    let scene = Scene {
        camera: Camera {
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
            up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
        },
        lights: vec![Light { position: Point::new(0.0, 0.0, 0.0), color: Color::new(1.0, 1.0, 1.0), intensity: 1.0 }],
        objects: vec![(sphere, material)]
    };
    raytrace(&scene, 200, 200, std::f64::consts::PI / 3.5);
}
