use nalgebra::Vector3;

use crate::types::PixelValueType;

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

    #[allow(clippy::trivially_copy_pass_by_ref)] // PixelValueType can change, only valid for u8/i8
    pub fn rgb(&self) -> (PixelValueType, PixelValueType, PixelValueType) {
        (self.data.x, self.data.y, self.data.z)
    }
}
