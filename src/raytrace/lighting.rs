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
    material.ambient
}

fn get_diffuse_color(light: &Light, material: &Material,
                     n: Vec3, l: Vec3, distance: f64) -> Color {
    // TODO: why do I need this abs
    (material.diffuse * (*light).color *
        n.dot(l).abs() * (*light).intensity / distance)
}

fn get_specular_color(material: &Material,
                      l: Vec3, v: Vec3, normal: Vec3) -> Color {
    let h = (v + l).normalized();
    (material.specular *
     (normal.dot(h).max(0.0).powf((*material).specular_exponent)))
}

pub fn get_color(scene: &Scene, materialobject: &MaterialObject,
                 ray: &Ray, t: f64) -> (Color, u64) {
    let (ref object, ref material) = *materialobject;
    let collision = ray.origin + (ray.direction * t);
    let normal = get_normal(object, collision);
    let ambient = get_ambient_color(&material);
    scene.lights.iter().fold((ambient, 0), |accum, light| {
        let (prev_color, prev_rays) = accum;
        let direction = light.position - collision;
        let new_ray = Ray {
            origin: collision,
            direction: direction
        };
        let distance = direction.mag_squared().sqrt();
        let l = direction.normalized();
        let v = new_ray.origin - new_ray.direction;

        match scene.intersects(&new_ray) {
            Some(_) => {
                (prev_color, prev_rays + 1)
            }
            None => {
                let diffuse = get_diffuse_color(&light, material, normal, l, distance);
                let specular = get_specular_color(material, l, v, normal);
                (prev_color + diffuse + specular, prev_rays + 1)
            }
        }
    })
}
