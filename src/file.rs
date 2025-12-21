use std::fs::File;
use std::io::{BufWriter, prelude::*};

pub struct Pixel(pub u8, pub u8, pub u8);

pub fn write_to_file(
    filename: &str,
    width: u32,
    height: u32,
    img: &[Pixel],
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_fmt(format_args!("P6 {} {} 255\n", width, height))?;

    let mut buffer = BufWriter::new(file);
    let mut total_pixels: u32 = width * height;
    for pixel in img {
        print!("\r{} pixels remaining to write              ", total_pixels);
        buffer.write_all(&[pixel.0, pixel.1, pixel.2])?;
        total_pixels -= 1;
    }
    println!("\nOutput written to {filename}");
    Ok(())
}
