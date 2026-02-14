use rt::camera::Camera;
use rt::file::ImageParameters;
use rt::objects::ObjectList;
use rt::objects::sphere::Sphere;
use rt::vec3::Point3;

fn main() -> std::io::Result<()> {
    // image constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let image_parameters = ImageParameters {
        width: IMAGE_WIDTH,
        height: IMAGE_HEIGHT,
    };

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

    let mut world: ObjectList = ObjectList::default();

    // small sphere
    world.add(&Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });

    // big sphere
    world.add(&Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });

    camera.render(&world, &image_parameters)?;

    Ok(())
}
