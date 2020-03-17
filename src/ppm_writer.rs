use std::fs::File;
use std::io::prelude::*;

use crate::pixel::Pixel;

pub fn write_ppm_file(filename: &str, data: &Vec<Vec<Pixel>>, max_value: usize) {
    let mut f = File::create(filename).expect("Unable to create file");
    let (x_length, y_length) = (data.len(), data[0].len());

    write_line_to_file(&mut f, "P3".to_owned());

    let string = x_length.to_string() + "\t" + &y_length.to_string();
    write_line_to_file(&mut f, string);

    let string = max_value.to_string();
    write_line_to_file(&mut f, string);

    for row in data {
        write_line_to_file(&mut f, vec_to_string(row, max_value, "\t"));
    }
}

fn write_line_to_file(f: &mut File, string: String) {
    f.write_all(string.as_bytes())
        .expect("Could not write string to file");
    if string.chars().rev().nth(0) != Some('\n') {
        f.write_all(b"\n").expect("error adding newline");
    }
}

fn vec_to_string(vector: &Vec<Pixel>, max_value: usize, delimiter: &str) -> String {
    let mut string = String::new();
    for pixel in vector {
        for sub in pixel {
            let value_as_usize = (sub * max_value as f32) as usize;
            string.push_str(&value_as_usize.to_string());
            string.push_str(delimiter);
        }
    }
    string
}
