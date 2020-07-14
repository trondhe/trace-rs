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

static X_SIZE: usize = 250;
static Y_SIZE: usize = 150;
static SAMPLES: usize = 50;
static MAX_BOUNCES: usize = 8;

fn main() {
    let material_metal_blank = ObjectType::Surface(Surface {
        roughness: 0.,
        attenuation: Vec3::new(0.95, 0.95, 0.95),
    });
    let _material_metal_matte = ObjectType::Surface(Surface {
        roughness: 0.85,
        attenuation: Vec3::new(0.80, 0.80, 0.80),
    });
    let material_grass_like = ObjectType::Surface(Surface {
        roughness: 0.95,
        attenuation: Vec3::new(0.65, 0.77, 0.01),
    });
    let material_red_matte = ObjectType::Surface(Surface {
        roughness: 1.,
        attenuation: Vec3::new(0.99, 0.20, 0.20),
    });
    let material_red_blank = ObjectType::Surface(Surface {
        roughness: 0.05,
        attenuation: Vec3::new(0.99, 0.20, 0.20),
    });
    let _light_source = ObjectType::LightSource(Light {
        intensity: Vec3::new(2., 1.5, 0.5),
    });

    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        0.,
        0.,
        -1.,
        0.3,
        material_metal_blank,
    )));
    world.push(Box::new(Sphere::new(0.6, 0., -1., 0.3, material_red_blank)));
    world.push(Box::new(Sphere::new(
        -0.6,
        0.,
        -1.,
        0.3,
        material_red_matte,
    )));
    world.push(Box::new(Sphere::new(
        0.,
        -100.3,
        -1.,
        100.,
        material_grass_like,
    )));
    // world.push(Sphere::new(10., 10., -25., 1., light_source));

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
