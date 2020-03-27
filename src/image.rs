use crate::Pixel;

#[allow(dead_code)]
pub struct Image {
    data: Vec<Pixel>,
    pub x_length: usize,
    pub y_length: usize,
}

impl Image {
    pub fn new(y_length: usize, x_length: usize) -> Self {
        Image {
            data: vec![Pixel::new(0,0,0,);y_length * x_length],
            x_length,
            y_length,
        }
    }

    pub fn write_x_vec(&mut self, y_index: usize, x_vec: &Vec<Pixel>) {
        assert_eq!(
            x_vec.len(),
            self.y_length,
            "Given row vector of length {} is different from image x_length {}",
            x_vec.len(),
            self.x_length
        );
        let vec_index = self.y_length * y_index;
        assert!(
            self.y_length * self.x_length >= vec_index + x_vec.len(),
            "Size of image {} is not big enough for index + row length {}",
            self.data.len(),
            vec_index + x_vec.len()
        );

        self.data[vec_index..vec_index+x_vec.len()].copy_from_slice(&x_vec);  
    }

    pub fn get_x_vec(&self, y_index: usize) -> Vec<Pixel> {
        let vec_index = self.y_length * y_index;
        let mut x_vec: Vec<Pixel> = Vec::with_capacity(self.y_length);
        x_vec.extend_from_slice(&self.data[vec_index..vec_index + self.y_length]);
        x_vec
    }
}
