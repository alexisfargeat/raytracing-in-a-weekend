use rt::file::write_to_file;
use rt::vec3::Color;

fn main() -> std::io::Result<()> {
    let mut image: [[Color; 256]; 256] = [[Color::default(); 256]; 256];

    for (i, row) in image.iter_mut().enumerate() {
        for (j, pixel) in row.iter_mut().enumerate() {
            *pixel = Color {
                x: i as f64 / 255.0,
                y: j as f64 / 255.0,
                z: 0.0,
            };
        }
    }
    write_to_file("test.ppm", &image)?;
    Ok(())
}
