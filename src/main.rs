use rt::file::{Pixel, write_to_file};

fn main() -> std::io::Result<()> {
    let mut image: Vec<Pixel> = Vec::new();
    for i in 0..=255 {
        for j in 0..=255 {
            image.push(Pixel(i, j, 0));
        }
    }
    write_to_file("test.ppm", 256, 256, image.as_slice())?;
    println!("Hello, world!");
    Ok(())
}
