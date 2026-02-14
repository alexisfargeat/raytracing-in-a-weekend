use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::prelude::Write;

use crate::utils::Interval;
use crate::vec3::Color;

pub struct ImageParameters {
    pub width: usize,
    pub height: usize,
}

const COLOR_INTERVAL: Interval = Interval::new(0.0, 0.999999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component.is_sign_positive() {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn linear_to_u8(linear_component: f64) -> u8 {
    (linear_to_gamma(COLOR_INTERVAL.clamp(linear_component)) * 256.0) as u8
}

pub fn write_image_to_file<T: Fn(usize, usize) -> Color>(
    filename: &str,
    params: &ImageParameters,
    color_function: T,
    samples_per_pixel: usize,
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
            if total_pixels.is_multiple_of(2000) {
                print!("\r{} pixels remaining to write              ", total_pixels);
                io::stdout().flush()?;
            }

            let mut color_sum: Color = Color::default();
            for _ in 0..samples_per_pixel {
                color_sum = color_sum + color_function(column_number, row_number);
            }

            let pixel_color = color_sum / (samples_per_pixel as f64);
            buffer.write_all(&[
                linear_to_u8(pixel_color.x),
                linear_to_u8(pixel_color.y),
                linear_to_u8(pixel_color.z),
            ])?;
            total_pixels -= 1;
        }
    }

    println!("\rAll pixels were written successfully !              ");
    println!("Output written to {filename}");
    Ok(())
}
