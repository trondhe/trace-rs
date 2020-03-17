mod pixel;
mod ppm_writer;

use pixel::Pixel;
use ppm_writer::write_ppm_file;

fn main() {
    let data: Vec<Vec<Pixel>> = vec![
        vec![
            Pixel { r: 255, g: 0, b: 0 },
            Pixel { r: 0, g: 255, b: 0 },
            Pixel { r: 0, g: 0, b: 255 },
        ],
        vec![
            Pixel { r: 255, g: 0, b: 0 },
            Pixel { r: 0, g: 255, b: 0 },
            Pixel { r: 0, g: 0, b: 255 },
        ],
    ];
    write_ppm_file("output.ppm", &data, 255);
}
