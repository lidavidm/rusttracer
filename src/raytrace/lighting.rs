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

fn get_diffuse_color(scene: &Scene, material: &Material,
                     normal: Vec3, collision: Vec3) -> (Color, u64) {
    // TODO: This fold should really be in get_color
    (*scene).lights.iter().fold((Color::new(0.0, 0.0, 0.0), 0), |accum, light| {
        let (prev_color, prev_rays) = accum;
        let direction = light.position - collision;
        let ray = Ray { origin: collision + (direction * 0.01), direction: direction };
        let distance = ray.direction.mag_squared();
        (prev_color + ((*material).diffuse / distance), prev_rays + 1)
        // TODO: why are we self-intersecting?
        // match scene.intersects(&ray) {
        //     Some((_, t)) => {
        //         accum
        //     }
        //     None => {
        //         accum + (*material).diffuse
        //     }
        // }
    })
}

pub fn get_color(scene: &Scene, materialobject: &MaterialObject,
                 ray: &Ray, t: f64) -> (Color, u64) {
    let (ref object, ref material) = *materialobject;
    let collision = ray.origin + (ray.direction * t);
    let normal = get_normal(object, collision);
    let (diffuse, rays) = get_diffuse_color(scene, &material, normal, collision);
    (get_ambient_color(&material) + diffuse, rays)
}
