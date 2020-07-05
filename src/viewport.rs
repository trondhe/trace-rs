use rand::Rng;

use crate::ray::Ray;
use crate::types::{TraceValueType, Vec3};

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
        let horizontal_length: TraceValueType = 2.0;
        let aspect_ratio = x_size as TraceValueType / y_size as TraceValueType;
        let vertical_length: TraceValueType = horizontal_length / aspect_ratio;
        let vertical_point_start: TraceValueType = -vertical_length / 2.0;
        Viewport {
            x_size,
            y_size,
            point_origin: Vec3::new(0., 0., 0.),
            point_lower_left_corner: Vec3::new(-1., vertical_point_start, -1.),
            vec_horizontal: Vec3::new(horizontal_length, 0., 0.),
            vec_vertical: Vec3::new(0., vertical_length, 0.),
        }
    }

    pub fn get_ray(&self, x_index: usize, y_index: usize) -> Ray {
        // Random range from -1.0 to +1.0
        let mut rng = rand::thread_rng();
        let x_rand = rng.gen::<TraceValueType>() * 2. - 1.;
        let y_rand = rng.gen::<TraceValueType>() * 2. - 1.;

        // random offset for intra pixel/bucket sampling
        let index_length = 1.0 / self.x_size as TraceValueType;
        let u_offset: TraceValueType = index_length + (x_rand * index_length);
        let v_offset: TraceValueType = index_length + (y_rand * index_length);

        let u = u_offset + x_index as TraceValueType / self.x_size as TraceValueType;
        let v = v_offset + y_index as TraceValueType / self.y_size as TraceValueType;
        Ray::new(
            self.point_origin,
            self.point_lower_left_corner + u * self.vec_horizontal + v * self.vec_vertical,
        )
    }
}
