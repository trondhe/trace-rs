use crate::types::PixelValueType;
use crate::types::{TraceValueType, Vec3};
use crate::Pixel;

#[derive(Debug)]
pub struct Image {
    pub data: Vec<Pixel>,
    pub x_size: usize,
    pub y_size: usize,
}

impl Image {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Image {
            data: vec![
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                );
                x_size * y_size
            ],
            x_size,
            y_size,
        }
    }

    pub fn from_candela(
        light_values: &[Vec3],
        x_size: usize,
        y_size: usize,
        max_value: usize,
    ) -> Self {
        let mut image = Image::new(x_size, y_size);

        // let brightest_spot: TraceValueType = light_values
        //     .iter()
        //     .map(|vec| vec.max())
        //     .fold(0., |current_max: TraceValueType, v| current_max.max(v));
        for (index, light) in light_values.iter().enumerate() {
            let r = Image::clamp_integer(
                (light.x.sqrt()) * max_value as TraceValueType,
                0,
                PixelValueType::MAX as isize,
            ) as PixelValueType;
            let g = Image::clamp_integer(
                (light.y.sqrt()) * max_value as TraceValueType,
                0,
                PixelValueType::MAX as isize,
            ) as PixelValueType;
            let b = Image::clamp_integer(
                (light.z.sqrt()) * max_value as TraceValueType,
                0,
                PixelValueType::MAX as isize,
            ) as PixelValueType;
            // let r = ((light.x / brightest_spot) * max_value as TraceValueType) as PixelValueType;
            // let g = ((light.y / brightest_spot) * max_value as TraceValueType) as PixelValueType;
            // let b = ((light.z / brightest_spot) * max_value as TraceValueType) as PixelValueType;
            image.data[index] = Pixel::new(r, g, b);
        }
        image
    }

    fn clamp_integer(input: TraceValueType, min: isize, max: isize) -> isize {
        if input as isize > max {
            max
        } else if (input as isize) < min {
            min
        } else {
            input as isize
        }
    }

    pub fn write_x_vec(&mut self, y_index: usize, x_vec: &[Pixel]) {
        assert_eq!(
            x_vec.len(),
            self.x_size,
            "Given row vector of length {} is different from image x_size {}",
            x_vec.len(),
            self.x_size
        );
        let vec_index = self.x_size * y_index;
        assert!(
            self.y_size * self.x_size >= vec_index + x_vec.len(),
            "Size of image {} is not big enough for index + row length {}",
            self.data.len(),
            vec_index + x_vec.len()
        );

        self.data[vec_index..vec_index + x_vec.len()].copy_from_slice(&x_vec);
    }

    pub fn get_x_vec(&self, y_index: usize) -> Vec<Pixel> {
        let vec_index = self.x_size * y_index;
        let mut x_vec: Vec<Pixel> = Vec::with_capacity(self.x_size);
        x_vec.extend_from_slice(&self.data[vec_index..vec_index + self.x_size]);
        x_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_line4x3(line_number: usize) -> Vec<Pixel> {
        match line_number {
            0 => vec![
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    1 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
            1 => vec![
                Pixel::new(
                    0 as PixelValueType,
                    1 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    1 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    1 as PixelValueType,
                    0 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
            2 => vec![
                Pixel::new(
                    1 as PixelValueType,
                    1 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    1 as PixelValueType,
                    1 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
            _ => vec![
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
        }
    }

    fn create_test_image4x3() -> Image {
        let mut image = Image::new(4, 3);
        let line0 = create_line4x3(0);
        let line1 = create_line4x3(1);
        let line2 = create_line4x3(2);
        image.write_x_vec(0, &line0);
        image.write_x_vec(1, &line1);
        image.write_x_vec(2, &line2);
        image
    }

    fn create_line3x2(line_number: usize) -> Vec<Pixel> {
        match line_number {
            0 => vec![
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    1 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
            1 => vec![
                Pixel::new(
                    0 as PixelValueType,
                    1 as PixelValueType,
                    1 as PixelValueType,
                ),
                Pixel::new(
                    1 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    1 as PixelValueType,
                    0 as PixelValueType,
                    1 as PixelValueType,
                ),
            ],
            _ => vec![
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
                Pixel::new(
                    0 as PixelValueType,
                    0 as PixelValueType,
                    0 as PixelValueType,
                ),
            ],
        }
    }

    fn create_test_image3x2() -> Image {
        let mut image = Image::new(3, 2);
        let line0 = create_line3x2(0);
        let line1 = create_line3x2(1);
        image.write_x_vec(0, &line0);
        image.write_x_vec(1, &line1);
        image
    }

    #[test]
    fn get_x_vec_returns_with_correct_indexing3x2() {
        let image = create_test_image3x2();
        assert_eq!(image.get_x_vec(0), create_line3x2(0));
        assert_eq!(image.get_x_vec(1), create_line3x2(1));
    }

    #[test]
    fn get_x_vec_returns_with_correct_indexing4x3() {
        let image = create_test_image4x3();
        assert_eq!(image.get_x_vec(0), create_line4x3(0));
        assert_eq!(image.get_x_vec(1), create_line4x3(1));
        assert_eq!(image.get_x_vec(2), create_line4x3(2));
    }
}
