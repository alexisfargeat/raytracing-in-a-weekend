use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

use crate::utils::Interval;
use crate::vec3::Color;

pub struct ImageParameters {
    pub width: usize,
    pub height: usize,
}

const COLOR_INTERVAL: Interval = Interval::new(0.0, 0.999999);

pub fn write_image_to_file<T: Fn(usize, usize) -> Color>(
    filename: &str,
    params: &ImageParameters,
    color_function: T,
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    assert!(
        params.width > 0 && params.height > 0,
        "Image dimensions must be positive."
    );

    file.write_fmt(format_args!("P6 {} {} 255\n", params.width, params.height))?;

    let mut buffer = BufWriter::new(file);
    let mut total_pixels = params.width * params.height;
    for row_number in 0..params.height {
        for column_number in 0..params.width {
            print!("\r{} pixels remaining to write              ", total_pixels);
            let pixel_color = color_function(column_number, row_number);
            buffer.write_all(&[
                (COLOR_INTERVAL.clamp(pixel_color.x) * 256.0) as u8,
                (COLOR_INTERVAL.clamp(pixel_color.y) * 256.0) as u8,
                (COLOR_INTERVAL.clamp(pixel_color.z) * 256.0) as u8,
            ])?;
            total_pixels -= 1;
        }
    }

    println!("\rAll pixels were written successfully !              ");
    println!("Output written to {filename}");
    Ok(())
}
