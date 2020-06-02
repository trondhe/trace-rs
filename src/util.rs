use crate::image::Image;
use crate::pixel::Pixel;
use crate::ppm_writer::ppm_file_write;
use crate::types::PixelValueType;

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
