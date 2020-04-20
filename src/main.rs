extern crate nalgebra as na;

mod image;
mod pixel;
mod ppm_writer;
mod ray;
mod util;

use image::Image;
use na::Vector3;
use pixel::Pixel;
use ppm_writer::ppm_file_write;
use ray::{Point, PointValueType, Ray};
use util::{create_test_image, gradient};

static X_SIZE: usize = 400;
static Y_SIZE: usize = 200;

fn main() {
    let point_origin: Point = Vector3::new(0., 0., 0.);
    let point_lower_left_corner: Point = Vector3::new(-1., -1., -1.);
    let vec_horizontal: Point = Vector3::new(2., 0., 0.);
    let vec_vertical: Point = Vector3::new(0., 2., 0.);

    let mut image = Image::new(X_SIZE, Y_SIZE);
    let mut x_vec: Vec<Pixel> = Vec::with_capacity(X_SIZE);
    for y_index in 0..Y_SIZE {
        for x_index in 0..X_SIZE {
            let u = x_index as PointValueType / X_SIZE as PointValueType;
            let v = y_index as PointValueType / Y_SIZE as PointValueType;
            let ray = Ray::with_value(
                point_origin,
                point_lower_left_corner + u * vec_horizontal + v * vec_vertical,
            );
            x_vec.push(gradient(ray));
        }
        image.write_x_vec(y_index, &x_vec);
        x_vec.clear();
    }

    create_test_image();
    ppm_file_write("generated/output.ppm", &image, 255);
}
