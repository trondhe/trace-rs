use crate::object::HitableList;
use crate::tracer::Tracer;
use crate::types::{TraceValueType, Vec3};
use crate::viewport::Viewport;
// use rayon::prelude::*;

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
        (0..self.samples).for_each(|_| {
            let mut frame = vec![Vec3::new(0., 0., 0.); self.vp.y_size * self.vp.x_size];
            for y_index in 0..self.vp.y_size {
                for x_index in 0..self.vp.x_size {
                    let ray = self.vp.get_ray(x_index, y_index);
                    let index = self.vp.x_size * y_index + x_index;
                    frame[index] = self.tracer.trace(&ray, &hitable_list);
                }
            }
            self.sensor.store_frame(&frame);
        });
    }

    pub fn sensor_data(&self) -> &Vec<Vec3> {
        &self.sensor.light_values
    }
}

pub struct Sensor {
    light_values: Vec<Vec3>,
    samples: usize,
    pub x_size: usize,
    pub y_size: usize,
}

impl Sensor {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            light_values: vec![Vec3::new(0., 0., 0.); x_size * y_size],
            samples: 0,
            x_size,
            y_size,
        }
    }

    fn store_frame(&mut self, frame: &Vec<Vec3>) {
        assert!(self.light_values.len() == frame.len());
        let n = self.samples as TraceValueType;
        for index in 0..self.light_values.len() {
            let light_previous = self.light_values[index];
            let light_new = frame[index];
            self.light_values[index] = (light_new + n * light_previous) / (n + 1.);
        }
        self.samples += 1;
    }
}
