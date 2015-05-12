use vector::*;
use objects::*;


fn get_normal(object: &Object, collision: Vec3) -> Vec3 {
    match *object {
        Object::Sphere { center: c, radius: _ } => {
            (collision - c).normalized()
        }
    }
}

fn get_ambient_color(material: &Material) -> Color {
    (*material).ambient
}

fn get_diffuse_color(scene: &Scene, light: &Light,
                     material: &Material, ray: &Ray) -> Color {
    let distance = ray.direction.mag_squared();

    // TODO: why are we self-intersecting?
    let scale = (light.intensity / distance) * (match scene.intersects(&ray) {
        Some(_) => {
            0.9
        }
        None => {
            1.0
        }
    });

    (*material).diffuse * scale
}

fn get_specular_color(material: &Material,
                      l: Vec3, v: Vec3, normal: Vec3) -> Color {
    let h = (v + l).normalized();
    (*material).specular * (normal.dot(h).max(0.0).powf((*material).specular_exponent))
}

pub fn get_color(scene: &Scene, materialobject: &MaterialObject,
                 ray: &Ray, t: f64) -> (Color, u64) {
    let (ref object, ref material) = *materialobject;
    let collision = ray.origin + (ray.direction * t);
    let normal = get_normal(object, collision);
    let ambient = get_ambient_color(&material);
    (*scene).lights.iter().fold((ambient, 0), |accum, light| {
        let (prev_color, prev_rays) = accum;
        let direction = (*light).position - collision;
        let new_ray = Ray {
            origin: collision - (direction * 0.00001),
            direction: direction
        };
        let l = light.position - collision;
        let v = new_ray.origin - new_ray.direction;

        let diffuse = get_diffuse_color(scene, &light, material, &new_ray);
        let specular = get_specular_color(material, l, v, normal);
        (prev_color + diffuse + specular, prev_rays + 1)
    })
}
