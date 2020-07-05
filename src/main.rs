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

static X_SIZE: usize = 600;
static Y_SIZE: usize = 500;
static SAMPLES: usize = 100;
static MAX_BOUNCES: usize = 8;

fn main() {
    let material_blank_metal = ObjectType::Surface(Surface {
        roughness: 0.9999,
        attenuation: Vec3::new(0.99, 0.99, 0.99),
    });
    let material_matte_red = ObjectType::Surface(Surface {
        roughness: 0.2,
        attenuation: Vec3::new(0.99, 0.50, 0.50),
    });
    let light_source = ObjectType::LightSource(Light {
        intensity: Vec3::new(2., 1.5, 0.5),
    });

    let mut world = HitableList::new();
    world.push(Sphere::new(0., 0., -1., 0.3, material_blank_metal));
    world.push(Sphere::new(0.5, 0., -0.7, 0.15, material_matte_red));
    world.push(Sphere::new(0., -100.3, -1., 100., material_blank_metal));
    world.push(Sphere::new(10., 10., -25., 1., light_source));

    let mut camera = Camera::new(CameraConfig {
        x_size: X_SIZE,
        y_size: Y_SIZE,
        samples: SAMPLES,
        max_bounces: MAX_BOUNCES,
    });
    camera.capture(world);

    let image = Image::from_candela(camera.sensor_data(), X_SIZE, Y_SIZE, 255);
    create_test_image();
    ppm_file_write("generated/output.ppm", &image, PixelValueType::MAX as usize);
}
