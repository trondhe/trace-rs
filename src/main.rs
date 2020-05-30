extern crate nalgebra as na;

mod image;
mod object;
mod pixel;
mod ppm_writer;
mod ray;
mod types;
mod util;

use image::Image;
use pixel::Pixel;
use ppm_writer::ppm_file_write;
use ray::Ray;
use types::{Vec3, VecValueType};
use util::{color, create_test_image};

static X_SIZE: usize = 400;
static Y_SIZE: usize = 400;

fn main() {
    let point_origin: Vec3 = Vec3::new(0., 0., 0.);
    let point_lower_left_corner: Vec3 = Vec3::new(-1., -1., -1.);
    let vec_horizontal: Vec3 = Vec3::new(2., 0., 0.);
    let vec_vertical: Vec3 = Vec3::new(0., 2., 0.);

    let mut image = Image::new(X_SIZE, Y_SIZE);
    let mut x_vec: Vec<Pixel> = Vec::with_capacity(X_SIZE); //woo
    for y_index in 0..Y_SIZE {
        for x_index in 0..X_SIZE {
            let u = x_index as VecValueType / X_SIZE as VecValueType;
            let v = y_index as VecValueType / Y_SIZE as VecValueType;
            let ray = Ray::new(
                point_origin,
                point_lower_left_corner + u * vec_horizontal + v * vec_vertical,
            );
            x_vec.push(color(ray));
        }
        image.write_x_vec(y_index, &x_vec);
        x_vec.clear();
    }

    create_test_image();
    ppm_file_write("generated/output.ppm", &image, 255);
}
