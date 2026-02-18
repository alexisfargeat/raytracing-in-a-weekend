use rt::camera::Camera;
use rt::file::ImageParameters;
use rt::materials::dielectric::Dielectric;
use rt::materials::lambertian::Lambertian;
use rt::materials::metal::Metal;
use rt::objects::ObjectList;
use rt::objects::sphere::Sphere;
use rt::vec3::Color;
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
            y: 0.5,
            z: 10.0,
        },
        focal_length: 6.0,
        viewport_width: VIEWPORT_WIDTH,
        viewport_height: VIEWPORT_HEIGHT,
        samples_per_pixel: 50,
        max_depth: 10,
    };

    let red_lambertian = Lambertian {
        albedo: Color {
            x: 1.0,
            y: 0.2,
            z: 0.2,
        },
    };

    let blue_fuzzy_metal = Metal::new(
        Color {
            x: 0.2,
            y: 0.2,
            z: 0.8,
        },
        0.8,
    );

    let clear_dielectric = Dielectric::new(
        1.5,
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    );

    let mut world: ObjectList = ObjectList::default();

    // small sphere
    world.add(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        radius: 0.5,
        material: &red_lambertian,
    });

    // first metal sphere
    world.add(Sphere {
        center: Point3 {
            x: -1.5,
            y: 0.0,
            z: -2.0,
        },
        radius: 0.5,
        material: &blue_fuzzy_metal,
    });

    // glass sphere
    world.add(Sphere {
        center: Point3 {
            x: 1.5,
            y: 0.0,
            z: -2.0,
        },
        radius: 0.5,
        material: &clear_dielectric,
    });

    // ball behind the glass
    world.add(Sphere {
        center: Point3 {
            x: 1.0,
            y: 0.25,
            z: -5.0,
        },
        radius: 0.25,
        material: &red_lambertian,
    });

    // big sphere
    world.add(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: &red_lambertian,
    });

    camera.render(&world, &image_parameters)?;

    Ok(())
}
