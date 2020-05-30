use crate::image::Image;
use crate::object::Sphere;
use crate::pixel::Pixel;
use crate::types::{PixelValueType, Vec3};
use crate::{ppm_writer::ppm_file_write, ray::Ray};

#[allow(dead_code)]
pub fn create_test_image() {
    let mut image = Image::new(4, 3);
    let line1 = vec![
        Pixel::new(1., 0., 0.),
        Pixel::new(0., 1., 0.),
        Pixel::new(0., 0., 1.),
        Pixel::new(1., 1., 1.),
    ];
    let line2 = vec![
        Pixel::new(1., 1., 0.),
        Pixel::new(0., 1., 1.),
        Pixel::new(1., 0., 1.),
        Pixel::new(1., 1., 1.),
    ];
    let line3 = vec![
        Pixel::new(0., 0., 0.),
        Pixel::new(0.33, 0.33, 0.33),
        Pixel::new(0.66, 0.66, 0.66),
        Pixel::new(1., 1., 1.),
    ];
    image.write_x_vec(0, &line1);
    image.write_x_vec(1, &line2);
    image.write_x_vec(2, &line3);

    ppm_file_write("generated/testimage.ppm", &image, 255);
}

pub fn color(ray: Ray) -> Pixel {
    let sphere = Sphere::new(0., 0., -1., 0.5);
    if sphere.hit(ray) {
        return Pixel::new(1., 0., 0.);
    }
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    let vector = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.);
    Pixel::new(
        vector.x as PixelValueType,
        vector.y as PixelValueType,
        vector.z as PixelValueType,
    )
}
