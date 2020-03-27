extern crate nalgebra as na;

mod image;
mod pixel;
mod ppm_writer;

use image::Image;
use na::Vector3;
use pixel::{Pixel, PixelValueType};
use ppm_writer::ppm_file_write;

static X_SIZE: usize = 200;
static Y_SIZE: usize = 100;

type VecValueType = f32;
struct Ray {
    a: Vector3<VecValueType>,
    b: Vector3<VecValueType>,
}
#[allow(dead_code)]
impl Ray {
    fn new() -> Self {
        Ray {
            a: Vector3::new(0., 0., 0.),
            b: Vector3::new(0., 0., 0.),
        }
    }

    fn with_value(origin: Vector3<VecValueType>, direction: Vector3<VecValueType>) -> Self {
        Ray {
            a: origin,
            b: direction,
        }
    }

    fn point_at_parameter(&self, t: VecValueType) -> Vector3<VecValueType> {
        self.a + t * self.b
    }

    fn origin(&self) -> Vector3<VecValueType> {
        self.a
    }

    fn direction(&self) -> Vector3<VecValueType> {
        self.b
    }
}

fn gradient(ray: Ray) -> Vector3<VecValueType> {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.)
}

fn vector_to_pixel(vector: Vector3<VecValueType>) -> Pixel {
    Pixel::new(
        vector.x as PixelValueType,
        vector.y as PixelValueType,
        vector.z as PixelValueType,
    )
}

fn main() {
    let point_origin: Vector3<VecValueType> = Vector3::new(0., 0., 0.);
    let point_lower_left_corner: Vector3<VecValueType> = Vector3::new(-2., -1., -1.);
    let vec_horizontal: Vector3<VecValueType> = Vector3::new(4., 0., 0.);
    let vec_vertical: Vector3<VecValueType> = Vector3::new(0., 2., 0.);

    let mut image = Image::new(X_SIZE, Y_SIZE);
    let mut x_vec: Vec<Pixel> = Vec::with_capacity(X_SIZE);
    for y_index in 0..Y_SIZE {
        for x_index in 0..X_SIZE {
            let u = y_index as VecValueType / X_SIZE as VecValueType;
            let v = x_index as VecValueType / Y_SIZE as VecValueType;
            let ray = Ray::with_value(
                point_origin,
                point_lower_left_corner + u * vec_horizontal + v * vec_vertical,
            );
            let color = gradient(ray);
            x_vec.push(vector_to_pixel(color));
        }
        image.write_x_vec(y_index, &x_vec);
        x_vec.clear();
    }

    ppm_file_write("output.ppm", &image, 255);
}
