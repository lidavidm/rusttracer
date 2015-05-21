use vector::*;
use objects::*;

/// Computes the normal vector of the object at the collision point.
fn get_normal(object: &Object, collision: Vec3) -> Vec3 {
    match *object {
        Object::Sphere { center: c, radius: _ } => {
            (collision - c).normalized()
        }
    }
}

/// Computes the ambient color of the material.
///
/// Though this is broken up into a separate function, the ambient color is
/// solely a function of the material, and does not depend on the light or
/// object.
fn get_ambient_color(material: &Material) -> Color {
    material.ambient
}

/// Compute the diffuse color of the object, according to the Lambertian model.
///
/// `n` is the normal vector to the object.
///
/// `l` is the vector from the intersection point to the light in consideration.
///
/// `distance` is more properly treated as a distance factor than the actual
/// distance - the calculation simply divides by this factor, so if
/// different attenuation behaviors are desired, the calling code can simply
/// pass appropriate values to this function.
fn get_diffuse_color(light: &Light, material: &Material,
                     n: Vec3, l: Vec3, distance: f64) -> Color {
    (material.diffuse * (*light).color *
        n.dot(l) * (*light).intensity / distance)
}

fn get_specular_color(material: &Material,
                      l: Vec3, v: Vec3, normal: Vec3) -> Color {
    let h = (v + l).normalized();
    (material.specular *
     (normal.dot(h).max(0.0).powf((*material).specular_exponent)))
}

fn trace_reflected_ray(origin: Point, incident_ray: &Ray, normal: Vec3) -> Ray {
    Ray {
        origin: origin,
        direction: (incident_ray.direction -
                    (normal * (incident_ray.direction.dot(normal) * 2.0)))
    }
}

fn get_object_color(scene: &Scene, materialobject: &MaterialObject,
                    ray: &Ray, t: f64, bounces: u8) -> (Color, u64) {
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
                let diffuse =
                    get_diffuse_color(&light, material, normal, l, distance);
                let specular =
                    get_specular_color(material, l, v, normal);
                if bounces > 0 {
                    let reflected_ray =
                        trace_reflected_ray(collision, ray, normal);
                    let (reflected_color, new_rays) =
                        get_color(scene, &reflected_ray, bounces - 1);
                    let reflected = reflected_color * material.specular;
                    (prev_color + diffuse + specular + reflected,
                     prev_rays + 1 + 1 + new_rays)
                }
                else {
                    (prev_color + diffuse + specular, prev_rays + 1)
                }
            }
        }
    })
}

// This should really use Color::new. A Color is secretly a vector.
// But the compiler doesn't like that.
const BACKGROUND_COLOR: Color = Color { x: 0.1, y: 0.1, z: 0.1 };

pub fn get_color(scene: &Scene, ray: &Ray, bounces: u8) -> (Color, u64) {
    match scene.intersects(&ray) {
        Some((materialobject, t)) => {
            get_object_color(scene, materialobject, ray, t, bounces)
        }
        None => {
            (BACKGROUND_COLOR, 0)
        }
    }
}
