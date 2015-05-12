extern crate image;

use std::path::Path;

use vector::*;
use objects::*;

pub type Image = image::RgbImage;

// This should really use Color::new. A Color is secretly a vector.
const BACKGROUND_COLOR: Color = Color { x: 0.4, y: 0.4, z: 0.4 };

pub fn write_image(image: Image, filename: &str) {
    let _ = image.save(&Path::new(filename));
}

pub fn raytrace(scene: &Scene, width: u32, height: u32, h_fov: f64) {
    let f_width = width as f64;
    let f_height = height as f64;
    let v_fov = h_fov * (f_height / f_width);

    let vw_per_pixel = 2.0 * (h_fov / 2.0).tan() / f_width;
    let vh_per_pixel = 2.0 * (v_fov / 2.0).tan() / f_height;

    let mut image = image::RgbImage::new(width, height);

    let norm_up = scene.camera.up.normalized();
    let norm_right = scene.camera.direction.cross(scene.camera.up).normalized();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let cast_ray = |dx: f64, dy: f64| {
            let fi = (x as f64) + dx;
            let fj = (y as f64) + dy;

            let u_rel = vw_per_pixel * (fi - (f_width / 2.0));
            let v_rel = vh_per_pixel * (fj - (f_height / 2.0));

            let pixel_loc = scene.camera.position + scene.camera.direction +
                (norm_right * u_rel) + (norm_up * v_rel);

            let ray = Ray {
                origin: scene.camera.position,
                direction: pixel_loc - scene.camera.position
            };

            match scene.intersects(&ray) {
                Some((materialobject, _)) => {
                    let (_, ref material) = *materialobject;
                    (*material).ambient
                }
                None => {
                    BACKGROUND_COLOR
                }
            }
        };
        let c1 = cast_ray(0.0, 0.0);
        let c2 = cast_ray(1.0, 0.0);
        let c3 = cast_ray(0.5, 0.5);
        let c4 = cast_ray(0.0, 1.0);
        let c5 = cast_ray(1.0, 1.0);

        *pixel = ((c1 + c2 + c3 + c4 + c5) / 5.0).to_rgb();
    }

    write_image(image, "test.png");
}
