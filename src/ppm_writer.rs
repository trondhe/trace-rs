use std::fs::File;
use std::io::prelude::*;

use crate::image::Image;
use crate::pixel::Pixel;

pub fn ppm_file_write(filename: &str, image: &Image, max_value: usize) {
    let mut f = File::create(filename).expect("Unable to create file");

    write_line_to_file(&mut f, "P3".to_owned());

    let string = image.x_length.to_string() + "\t" + &image.y_length.to_string();
    write_line_to_file(&mut f, string);

    let string = max_value.to_string();
    write_line_to_file(&mut f, string);

    for y_index in (0..image.y_length).rev().collect::<Vec<usize>>() {
        write_line_to_file(&mut f, pixel_to_string(&image.get_x_vec(y_index), "\t"));
    }
}

fn write_line_to_file(f: &mut File, string: String) {
    f.write_all(string.as_bytes())
        .expect("Could not write string to file");
    if string.chars().rev().next() != Some('\n') {
        f.write_all(b"\n").expect("error adding newline");
    }
}

fn pixel_to_string(vector: &[Pixel], delimiter: &str) -> String {
    let mut string = String::new();
    for pixel in vector {
        let (r, g, b) = pixel.rgb();

        string.push_str(&r.to_string());
        string.push_str(delimiter);

        string.push_str(&g.to_string());
        string.push_str(delimiter);

        string.push_str(&b.to_string());
        string.push_str(delimiter);
    }
    string
}
