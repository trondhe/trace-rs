use std::cmp::PartialOrd;

use crate::image::Image;
use crate::pixel::Pixel;
use crate::types::{PixelValueType, VecValueType};
use crate::{ppm_writer::ppm_file_write, ray::RayHit};

pub fn color(v: VecValueType, maybe_hit: Option<RayHit>) -> Pixel {
    if let Some(hit) = maybe_hit {
        let normal = hit.normal;
        Pixel::from_float(
            normal.x + 1.,
            normal.y + 1.,
            normal.z + 1.,
            PixelValueType::MAX as usize,
        )
    } else {
        let background_r = 1.0 - v + v * 0.5;
        let background_g = 1.0 - v + v * 0.7;
        let background_b = 1.0 - v + v * 1.0;
        Pixel::from_float(
            background_r,
            background_g,
            background_b,
            PixelValueType::MAX as usize,
        )
    }
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}

#[allow(dead_code)]
pub fn create_test_image() {
    let mut image = Image::new(4, 3);
    let line1 = vec![
        Pixel::new(
            3 as PixelValueType,
            0 as PixelValueType,
            0 as PixelValueType,
        ),
        Pixel::new(
            0 as PixelValueType,
            3 as PixelValueType,
            0 as PixelValueType,
        ),
        Pixel::new(
            0 as PixelValueType,
            0 as PixelValueType,
            3 as PixelValueType,
        ),
        Pixel::new(
            3 as PixelValueType,
            3 as PixelValueType,
            3 as PixelValueType,
        ),
    ];
    let line2 = vec![
        Pixel::new(
            3 as PixelValueType,
            3 as PixelValueType,
            0 as PixelValueType,
        ),
        Pixel::new(
            0 as PixelValueType,
            3 as PixelValueType,
            3 as PixelValueType,
        ),
        Pixel::new(
            3 as PixelValueType,
            0 as PixelValueType,
            3 as PixelValueType,
        ),
        Pixel::new(
            3 as PixelValueType,
            3 as PixelValueType,
            3 as PixelValueType,
        ),
    ];
    let line3 = vec![
        Pixel::new(
            0 as PixelValueType,
            0 as PixelValueType,
            0 as PixelValueType,
        ),
        Pixel::new(
            1 as PixelValueType,
            1 as PixelValueType,
            1 as PixelValueType,
        ),
        Pixel::new(
            2 as PixelValueType,
            2 as PixelValueType,
            2 as PixelValueType,
        ),
        Pixel::new(
            3 as PixelValueType,
            3 as PixelValueType,
            3 as PixelValueType,
        ),
    ];
    image.write_x_vec(0, &line1);
    image.write_x_vec(1, &line2);
    image.write_x_vec(2, &line3);

    ppm_file_write("generated/testimage.ppm", &image, 3);
}
