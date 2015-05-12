extern crate image;

use std::path::Path;

use vector::*;
use objects::*;

pub type Image = image::RgbImage;

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
                Some(_) => {
                    (255.0, 0.0, 255.0)
                }
                None => {
                    (100.0, 100.0, 100.0)
                }
            }
        };
        let (r1, g1, b1) = cast_ray(0.0, 0.0);
        let (r2, g2, b2) = cast_ray(1.0, 0.0);
        let (r3, g3, b3) = cast_ray(0.5, 0.5);
        let (r4, g4, b4) = cast_ray(0.0, 1.0);
        let (r5, g5, b5) = cast_ray(1.0, 1.0);

        *pixel = image::Rgb([((r1 + r2 + r3 + r4 + r5) / 5.0) as u8,
                             ((g1 + g2 + g3 + g4 + g5) / 5.0) as u8,
                             ((b1 + b2 + b3 + b4 + b5) / 5.0) as u8]);
    }

    write_image(image, "test.png");
}
