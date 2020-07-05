use crate::types::{TraceValueType, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn p(&self, t: TraceValueType) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
