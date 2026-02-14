use crate::file::ImageParameters;
use crate::file::write_image_to_file;
use crate::objects::ObjectList;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Camera {
    pub center: Point3,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub samples_per_pixel: usize,
}

impl Camera {
    fn viewport_u(&self) -> Vec3 {
        Vec3 {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        }
    }

    fn delta_u(&self, image_params: &ImageParameters) -> Vec3 {
        self.viewport_u() / image_params.width as f64
    }

    fn viewport_v(&self) -> Vec3 {
        Vec3 {
            x: 0.0,
            y: -self.viewport_height,
            z: 0.0,
        }
    }

    fn delta_v(&self, image_params: &ImageParameters) -> Vec3 {
        self.viewport_v() / image_params.height as f64
    }

    fn viewport_upper_left(&self) -> Point3 {
        self.center
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: self.focal_length,
            }
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0
    }

    fn pixel_center(&self, x: usize, y: usize, image_params: &ImageParameters) -> Point3 {
        self.viewport_upper_left()
            + (x as f64 + 0.5) * self.delta_u(image_params)
            + (y as f64 + 0.5) * self.delta_v(image_params)
    }

    fn pixel_position_randomized(
        &self,
        x: usize,
        y: usize,
        image_params: &ImageParameters,
    ) -> Point3 {
        self.pixel_center(x, y, image_params)
            + (rand::random::<f64>() - 0.5) * self.delta_u(image_params)
            + (rand::random::<f64>() - 0.5) * self.delta_v(image_params)
    }

    pub fn render(
        &self,
        world: &ObjectList,
        image_params: &ImageParameters,
    ) -> std::io::Result<()> {
        pub fn color(normal_vector: Vec3) -> Color {
            0.5 * Color {
                x: normal_vector.x + 1.0,
                y: normal_vector.y + 1.0,
                z: normal_vector.z + 1.0,
            }
        }

        let color_function = |x: usize, y: usize| -> Color {
            let ray_direction: Vec3 =
                self.pixel_position_randomized(x, y, image_params) - self.center;

            let ray = Ray {
                origin: self.center,
                direction: ray_direction,
            };

            let hit = world.hit(&ray, Interval::new(0.0, f64::MAX));

            match hit {
                None => ray.color(),
                Some(hit_record) => color(hit_record.normal),
            }
        };

        write_image_to_file(
            "test.ppm",
            image_params,
            color_function,
            self.samples_per_pixel,
        )?;

        Ok(())
    }
}
