extern crate image;

use std::path::Path;

use vector::*;
use objects::*;

pub type Image = image::RgbImage;

pub struct Scene {
    lights: Vec<Light>,
    objects: Vec<Object>
}

pub fn write_image(image: Image, filename: &str) {
    let _ = image.save(&Path::new(filename));
}
