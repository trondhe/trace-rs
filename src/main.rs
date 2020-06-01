extern crate nalgebra as na;

mod image;
mod object;
mod object_list;
mod pixel;
mod ppm_writer;
mod ray;
mod types;
mod util;
mod viewport;

use image::Image;

use object::Sphere;
use object_list::HitableList;
use pixel::Pixel;
use ppm_writer::ppm_file_write;
use ray::Ray;
use types::{PixelValueType, VecValueType};
use util::{color, create_test_image};
use viewport::Viewport;

static X_SIZE: usize = 400;
static Y_SIZE: usize = 320;

fn main() {
    let vp = Viewport::new(X_SIZE, Y_SIZE);
    let mut image = Image::new(X_SIZE, Y_SIZE);
    let mut hitable_list = HitableList::new();
    hitable_list.push(Sphere::new(0., 0., -1., 0.5));
    hitable_list.push(Sphere::new(0., -100.5, -1., 100.));

    let mut x_vec: Vec<Pixel> = Vec::with_capacity(X_SIZE);
    for y_index in 0..vp.y_size {
        for x_index in 0..vp.x_size {
            let u = x_index as VecValueType / X_SIZE as VecValueType;
            let v = y_index as VecValueType / Y_SIZE as VecValueType;
            let ray = Ray::new(
                vp.point_origin,
                vp.point_lower_left_corner + u * vp.vec_horizontal + v * vp.vec_vertical,
            );
            let maybe_hit = hitable_list.find_foreground_hit(ray);
            x_vec.push(color(v, maybe_hit));
        }
        image.write_x_vec(y_index, &x_vec);
        x_vec.clear();
    }

    create_test_image();
    ppm_file_write("generated/output.ppm", &image, PixelValueType::MAX as usize);
}
