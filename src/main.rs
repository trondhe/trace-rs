extern crate nalgebra as na;

mod camera;
mod image;
mod object;
mod pixel;
mod ppm_writer;
mod ray;
mod tracer;
mod types;
mod util;
mod viewport;

use image::Image;

use camera::{Camera, CameraConfig};
use object::{HitableList, Sphere};
use pixel::Pixel;
use ppm_writer::ppm_file_write;
use types::PixelValueType;
use util::create_test_image;

static X_SIZE: usize = 500;
static Y_SIZE: usize = 300;
static SAMPLES: usize = 500;
static MAX_BOUNCES: usize = 10;
static MAX_TRACE_LENGTH: f32 = 10000.;
// static X_SIZE: usize = 100;
// static Y_SIZE: usize = 75;
// static SAMPLES: usize = 10;
// static MAX_BOUNCES: usize = 10;
// static MAX_TRACE_LENGTH: f32 = 10000.;

fn main() {
    let mut world = HitableList::new();
    world.push(Sphere::new(0., 0., -1., 0.3));
    world.push(Sphere::new(0., -100.5, -1., 100.));

    let mut camera = Camera::new(CameraConfig {
        x_size: X_SIZE,
        y_size: Y_SIZE,
        samples: SAMPLES,
        max_bounces: MAX_BOUNCES,
        max_trance_length: MAX_TRACE_LENGTH,
    });
    camera.capture(world);

    let image = Image::from_candela(camera.sensor_data(), X_SIZE, Y_SIZE, 255);
    create_test_image();
    ppm_file_write("generated/output.ppm", &image, PixelValueType::MAX as usize);
}
