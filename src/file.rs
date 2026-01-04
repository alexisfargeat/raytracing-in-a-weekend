use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

use crate::vec3::Color;

pub fn write_to_file<const W: usize, const H: usize>(
    filename: &str,
    img: &[[Color; H]; W],
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    assert!(W > 0 && H > 0);

    file.write_fmt(format_args!("P6 {} {} 255\n", W, H))?;

    let mut buffer = BufWriter::new(file);
    let mut total_pixels = W * H;
    for row in img {
        for pixel in row.iter() {
            print!("\r{} pixels remaining to write              ", total_pixels);
            buffer.write_all(&[
                (pixel.x * 255.999) as u8,
                (pixel.y * 255.999) as u8,
                (pixel.z * 255.999) as u8,
            ])?;
            total_pixels -= 1;
        }
    }
    println!("\rAll pixels were written successfully !              ");
    println!("Output written to {filename}");
    Ok(())
}
