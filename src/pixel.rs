use nalgebra::Vector3;

use crate::types::PixelValueType;
use crate::util::clamp;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    data: Vector3<PixelValueType>,
}

impl Pixel {
    pub fn new(r: PixelValueType, g: PixelValueType, b: PixelValueType) -> Self {
        Pixel {
            data: Vector3::new(r, g, b),
        }
    }

    pub fn from_float(r: f32, g: f32, b: f32, scale_by: usize) -> Self {
        let r = clamp(r, 0., 1.);
        let g = clamp(g, 0., 1.);
        let b = clamp(b, 0., 1.);
        let r = (r * scale_by as f32) as PixelValueType;
        let g = (g * scale_by as f32) as PixelValueType;
        let b = (b * scale_by as f32) as PixelValueType;
        Self {
            data: Vector3::new(r, g, b),
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)] // PixelValueType can change, only valid for u8/i8
    pub fn rgb(&self) -> (PixelValueType, PixelValueType, PixelValueType) {
        (self.data.x, self.data.y, self.data.z)
    }
}
