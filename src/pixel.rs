extern crate nalgebra as na;
use na::Vector3;

pub type PixelValueType = u8;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    data: Vector3<PixelValueType>,
}
#[allow(dead_code)]
impl Pixel {
    pub fn new(r: PixelValueType, g: PixelValueType, b: PixelValueType) -> Self {
        Pixel {
            data: Vector3::new(r, g, b),
        }
    }

    pub fn r(&self) -> PixelValueType {
        self.data.x
    }

    pub fn g(&self) -> PixelValueType {
        self.data.y
    }

    pub fn b(&self) -> PixelValueType {
        self.data.z
    }

    pub fn rgb(&self) -> (PixelValueType, PixelValueType, PixelValueType) {
        (self.data.x, self.data.y, self.data.z)
    }

    pub fn r_set(&mut self, value: PixelValueType) {
        self.data.x = value;
    }

    pub fn g_set(&mut self, value: PixelValueType) {
        self.data.y = value;
    }

    pub fn b_set(&mut self, value: PixelValueType) {
        self.data.z = value;
    }
}
