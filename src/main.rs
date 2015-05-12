mod vector;
mod objects;
mod raytrace;

use vector::*;
use objects::*;

fn main() {
    let ray = Ray { origin: Vec3::new(0.0, 0.0, 0.0),
                    direction: Vec3::new(0.0, 0.0, 1.0) };
    let sphere = Object::Sphere { center: Vec3::new(0.0, 0.0, 5.0),
                                  radius: 2.0 };

    match intersects(&ray, &sphere) {
        Some(_) => { println!("They intersect!") }
        None => { println!("They don't intersect!") }
    }

    println!("{:?}", ray);
}
