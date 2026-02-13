use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::VecOps;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let unit_direction = self.direction.unit_vector();

        // get blend coefficient by having y coordinates between 0 and 1
        let blend_coef = 0.5 * (unit_direction.y + 1.0);

        let white = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let light_blue: Vec3 = Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        (1.0 - blend_coef) * white + blend_coef * light_blue
    }
}
