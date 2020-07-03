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
use object::{HitableList, Light, ObjectType, Sphere, Surface};
use pixel::Pixel;
use ppm_writer::ppm_file_write;
use types::{PixelValueType, Vec3};
use util::create_test_image;

static X_SIZE: usize = 200;
static Y_SIZE: usize = 200;
static SAMPLES: usize = 500;
static MAX_BOUNCES: usize = 8;
static MAX_TRACE_LENGTH: f32 = 10000.;

fn main() {
    let mut world = HitableList::new();
    world.push(Sphere::new(
        0.,
        0.,
        -1.,
        0.3,
        ObjectType::Surface(Surface {
            roughness: 0.,
            attenuation: Vec3::new(0.6, 0.6, 0.6),
        }),
    ));
    world.push(Sphere::new(
        0.,
        -100.5,
        -1.,
        100.,
        ObjectType::Surface(Surface {
            roughness: 0.,
            attenuation: Vec3::new(0.6, 0.6, 0.6),
        }),
    ));
    world.push(Sphere::new(
        0.,
        100.,
        -10.,
        10.,
        ObjectType::LightSource(Light {
            intensity: Vec3::new(100., 100., 100.),
        }),
    ));

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
