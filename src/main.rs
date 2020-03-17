use std::fs::File;
use std::io::prelude::*;

struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}

impl Pixel {
    #[allow(dead_code)]
    fn new() -> Self {
        Pixel { r: 0, g: 0, b: 0 }
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

struct PixelIntoIterator {
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

struct PixelIterator<'a> {
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

fn main() {
    let data: Vec<Vec<Pixel>> = vec![vec![
        Pixel { r: 255, g: 0, b: 0 },
        Pixel { r: 0, g: 255, b: 0 },
        Pixel { r: 0, g: 0, b: 255 },
    ]];
    write_ppm_file("output.ppm", &data, 255);
}

fn write_ppm_file(filename: &str, data: &Vec<Vec<Pixel>>, max_value: u32) {
    let mut f = File::create(filename).expect("Unable to create file");
    let (x_length, y_length) = (data.len(), data[0].len());

    write_line_to_file(&mut f, "P3".to_owned());

    let string = x_length.to_string() + "\t" + &y_length.to_string();
    write_line_to_file(&mut f, string);

    let string = max_value.to_string();
    write_line_to_file(&mut f, string);

    for row in data {
        write_line_to_file(&mut f, vec_to_string(row, "\t"));
    }
}

fn write_line_to_file(f: &mut File, string: String) {
    f.write_all(string.as_bytes())
        .expect("Could not write string to file");
    if string.chars().rev().nth(0) != Some('\n') {
        f.write_all(b"\n").expect("error adding newline");
    }
}

fn vec_to_string(vector: &Vec<Pixel>, delimiter: &str) -> String {
    let mut string = String::new();
    for pixel in vector {
        for sub in pixel {
            string.push_str(&sub.to_string());
            string.push_str(delimiter);
        }
    }
    string
}
