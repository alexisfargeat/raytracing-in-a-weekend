use crate::file::ImageParameters;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Camera {
    pub center: Point3,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
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

    pub fn pixel_center(&self, x: usize, y: usize, image_params: &ImageParameters) -> Point3 {
        self.viewport_upper_left()
            + x as f64 * self.delta_u(image_params)
            + y as f64 * self.delta_v(image_params)
    }
}
