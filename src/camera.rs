use crate::object::HitableList;
use crate::ray::RayHit;
use crate::types::Vec3;
use crate::viewport::Viewport;

pub struct Camera {
    vp: Viewport,
    sensor: Sensor,
    samples: usize,
}

impl Camera {
    pub fn new(x_size: usize, y_size: usize, samples: usize) -> Self {
        Self {
            vp: Viewport::new(x_size, y_size),
            sensor: Sensor::new(x_size, y_size),
            samples,
        }
    }

    pub fn capture(&mut self, hitable_list: HitableList) {
        for y_index in 0..self.vp.y_size {
            for x_index in 0..self.vp.x_size {
                for _ in 0..self.samples {
                    let ray = self.vp.get_ray(x_index, y_index);
                    let maybe_hit = hitable_list.find_foreground_hit(ray);
                    self.sensor.store(x_index, y_index, maybe_hit);
                }
            }
        }
    }

    pub fn sensor_data(&self) -> &Vec<Vec3> {
        &self.sensor.light_values
    }
}

pub struct Sensor {
    light_values: Vec<Vec3>,
    samples: Vec<usize>,
    pub x_size: usize,
    pub y_size: usize,
}

impl Sensor {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            light_values: vec![Vec3::new(0., 0., 0.); x_size * y_size],
            samples: vec![0; x_size * y_size],
            x_size,
            y_size,
        }
    }

    fn store(&mut self, x_index: usize, y_index: usize, maybe_hit: Option<RayHit>) {
        let index = self.x_size * y_index + x_index;
        if let Some(hit) = maybe_hit {
            let normal = hit.normal;
            self.light_values[index] = Vec3::new(
                (normal.x + 1.) / 2.,
                (normal.y + 1.) / 2.,
                (normal.z + 1.) / 2.,
            );
        } else {
            // Background
            let v = y_index as f32 / self.y_size as f32;
            let background_r = 1.0 - v + v * 0.5;
            let background_g = 1.0 - v + v * 0.7;
            let background_b = 1.0 - v + v * 1.0;
            self.light_values[index] = Vec3::new(background_r, background_g, background_b);
            self.samples[index] += 1;
        }
    }
}
