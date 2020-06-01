use crate::types::{Vec3, VecValueType};

pub struct Viewport {
    pub x_size: usize,
    pub y_size: usize,
    pub point_origin: Vec3,
    pub point_lower_left_corner: Vec3,
    pub vec_horizontal: Vec3,
    pub vec_vertical: Vec3,
}

impl Viewport {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        let horizontal_length: VecValueType = 2.0;
        let aspect_ratio = x_size as VecValueType / y_size as VecValueType;
        let vertical_length: VecValueType = horizontal_length / aspect_ratio;
        let vertical_point_start: VecValueType = -vertical_length / 2.0;
        Viewport {
            x_size,
            y_size,
            point_origin: Vec3::new(0., 0., 0.),
            point_lower_left_corner: Vec3::new(-1., vertical_point_start, -1.),
            vec_horizontal: Vec3::new(horizontal_length, 0., 0.),
            vec_vertical: Vec3::new(0., vertical_length, 0.),
        }
    }
}
