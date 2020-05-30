use crate::Pixel;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Image {
    pub data: Vec<Pixel>,
    pub x_length: usize,
    pub y_length: usize,
}

impl Image {
    pub fn new(x_length: usize, y_length: usize) -> Self {
        Image {
            data: vec![Pixel::new(0., 0., 0.,); x_length * y_length],
            x_length,
            y_length,
        }
    }

    pub fn write_x_vec(&mut self, y_index: usize, x_vec: &[Pixel]) {
        assert_eq!(
            x_vec.len(),
            self.x_length,
            "Given row vector of length {} is different from image x_length {}",
            x_vec.len(),
            self.x_length
        );
        let vec_index = self.x_length * y_index;
        assert!(
            self.y_length * self.x_length >= vec_index + x_vec.len(),
            "Size of image {} is not big enough for index + row length {}",
            self.data.len(),
            vec_index + x_vec.len()
        );

        self.data[vec_index..vec_index + x_vec.len()].copy_from_slice(&x_vec);
    }

    pub fn get_x_vec(&self, y_index: usize) -> Vec<Pixel> {
        let vec_index = self.x_length * y_index;
        let mut x_vec: Vec<Pixel> = Vec::with_capacity(self.x_length);
        x_vec.extend_from_slice(&self.data[vec_index..vec_index + self.x_length]);
        x_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_line4x3(line_number: usize) -> Vec<Pixel> {
        match line_number {
            0 => vec![
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 1.),
                Pixel::new(0., 1., 0.),
                Pixel::new(0., 0., 0.),
            ],
            1 => vec![
                Pixel::new(0., 1., 1.),
                Pixel::new(1., 0., 0.),
                Pixel::new(1., 0., 1.),
                Pixel::new(0., 0., 0.),
            ],
            2 => vec![
                Pixel::new(1., 1., 0.),
                Pixel::new(1., 1., 1.),
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
            ],
            _ => vec![
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
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
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 1.),
                Pixel::new(0., 1., 0.),
            ],
            1 => vec![
                Pixel::new(0., 1., 1.),
                Pixel::new(1., 0., 0.),
                Pixel::new(1., 0., 1.),
            ],
            _ => vec![
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
                Pixel::new(0., 0., 0.),
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
