use vector::*;
use objects::*;


fn get_normal(object: &Object, collision: Vec3) -> Vec3 {
    match *object {
        Object::Sphere { center: c, radius: r } => {
            (collision - c).normalized()
        }
    }
}

fn get_ambient_color(material: &Material) -> Color {
    (*material).ambient
}

fn get_diffuse_color(scene: &Scene, material: &Material, collision: Point,
                     light: &Light) -> Color {
    let direction = (*light).position - collision;
    let ray = Ray {
        origin: collision + (direction * 0.00001),
        direction: direction
    };

    let distance = ray.direction.mag_squared();

    // TODO: why are we self-intersecting?
    let scale = (light.intensity / distance) * (match scene.intersects(&ray) {
        Some((_, t)) => {
            0.9
        }
        None => {
            1.0
        }
    });

    (*material).diffuse * scale
}

pub fn get_color(scene: &Scene, materialobject: &MaterialObject,
                 ray: &Ray, t: f64) -> (Color, u64) {
    let (ref object, ref material) = *materialobject;
    let collision = ray.origin + (ray.direction * t);
    let normal = get_normal(object, collision);
    let ambient = get_ambient_color(&material);
    (*scene).lights.iter().fold((ambient, 0), |accum, light| {
        let (prev_color, prev_rays) = accum;
        let diffuse = get_diffuse_color(scene, material, collision, &light);

        (prev_color + diffuse, prev_rays + 1)
    })
}
