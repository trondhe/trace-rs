pub struct Pixel {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Pixel {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Pixel { r: 0, g: 0, b: 0 }
    }

    pub fn with_values(r: u32, g: u32, b: u32) -> Self {
        Pixel { r, g, b }
    }
}

impl IntoIterator for Pixel {
    type Item = u32;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            pixel: self,
            index: 0,
        }
    }
}

pub struct PixelIntoIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a Pixel {
    type Item = u32;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            pixel: self,
            index: 0,
        }
    }
}

pub struct PixelIterator<'a> {
    pixel: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
