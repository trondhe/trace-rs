use crate::object::HitableList;
use crate::tracer::Tracer;
use crate::types::{TraceValueType, Vec3};
use crate::viewport::Viewport;
use rayon::prelude::*;

pub struct Camera {
    vp: Viewport,
    sensor: Sensor,
    tracer: Tracer,
    samples: usize,
}

pub struct CameraConfig {
    pub y_size: usize,
    pub x_size: usize,
    pub samples: usize,
    pub max_bounces: usize,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        Self {
            vp: Viewport::new(config.x_size, config.y_size),
            sensor: Sensor::new(config.x_size, config.y_size),
            tracer: Tracer::new(config.max_bounces),
            samples: config.samples,
        }
    }

    pub fn capture(&mut self, hitable_list: HitableList) {
        (0..self.vp.y_size).for_each(|y_index| {
            for x_index in 0..self.vp.x_size {
                for _ in 0..self.samples {
                    let ray = self.vp.get_ray(x_index, y_index);
                    let colour = self.tracer.trace(&ray, &hitable_list);
                    self.sensor.store(x_index, y_index, colour);
                }
            }
        });
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

    fn store(&mut self, x_index: usize, y_index: usize, color: Vec3) {
        let index = self.x_size * y_index + x_index;
        let n = self.samples[index] as TraceValueType;
        let previous_light = self.light_values[index];
        self.light_values[index] = (color + n * previous_light) / (n + 1.);
        self.samples[index] += 1;
    }
}
