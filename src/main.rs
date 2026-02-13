use rt::camera::Camera;
use rt::file::{ImageParameters, write_image_to_file};
use rt::ray::Ray;
use rt::vec3::{Color, Point3, Vec3};

fn main() -> std::io::Result<()> {
    // image constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64) * VIEWPORT_HEIGHT;
    let camera: Camera = Camera {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        focal_length: 1.0,
        viewport_width: VIEWPORT_WIDTH,
        viewport_height: VIEWPORT_HEIGHT,
    };

    let image_parameters = ImageParameters {
        width: IMAGE_WIDTH,
        height: IMAGE_HEIGHT,
    };

    let color_function = |x: usize, y: usize| -> Color {
        let ray_direction: Vec3 = camera.pixel_center(x, y, &image_parameters) - camera.center;

        let ray = Ray {
            origin: camera.center,
            direction: ray_direction,
        };

        ray.color()
    };

    write_image_to_file("test.ppm", &image_parameters, color_function)?;

    Ok(())
}
