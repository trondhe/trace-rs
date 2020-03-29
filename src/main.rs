extern crate nalgebra as na;

mod image;
mod pixel;
mod ppm_writer;
mod ray;

use image::Image;
use na::Vector3;
use pixel::{Pixel, PixelValueType};
use ppm_writer::ppm_file_write;
use ray::{Ray, Point, PointValueType};

static X_SIZE: usize = 20;
static Y_SIZE: usize = 10;

fn gradient(ray: Ray) -> Pixel {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    let vector = (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.);
    Pixel::new(
        vector.x as PixelValueType,
        vector.y as PixelValueType,
        vector.z as PixelValueType,
    )
}

fn main() {
    let point_origin: Point = Vector3::new(0., 0., 0.);
    let point_lower_left_corner: Point = Vector3::new(-1., -1., -1.);
    let vec_horizontal: Point = Vector3::new(2., 0., 0.);
    let vec_vertical: Point = Vector3::new(0., 2., 0.);

    let mut image = Image::new(X_SIZE, Y_SIZE);
    let mut x_vec: Vec<Pixel> = Vec::with_capacity(X_SIZE);
    for y_index in 0..Y_SIZE {
        for x_index in 0..X_SIZE {
            let u = y_index as PointValueType / Y_SIZE as PointValueType;
            let v = x_index as PointValueType / X_SIZE as PointValueType;
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

#[allow(dead_code)]
fn create_test_image() {
    let mut image = Image::new(4,3);
    let line1 = vec![Pixel::new(1.,0.,0.,),Pixel::new(0.,1.,0.,),Pixel::new(0.,0.,1.,),Pixel::new(1.,1.,1.,)];
    let line2 = vec![Pixel::new(1.,1.,0.,),Pixel::new(0.,1.,1.,),Pixel::new(1.,0.,1.,),Pixel::new(1.,1.,1.,)];
    let line3 = vec![Pixel::new(0.,0.,0.,),Pixel::new(0.33,0.33,0.33,),Pixel::new(0.66,0.66,0.66,),Pixel::new(1.,1.,1.,)];
    image.write_x_vec(0, &line1);
    image.write_x_vec(1, &line2);
    image.write_x_vec(2, &line3);    
    
    ppm_file_write("generated/testimage.ppm", &image, 255);
}
