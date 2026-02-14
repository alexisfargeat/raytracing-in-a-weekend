use crate::objects::ObjectList;
use crate::utils::Interval;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::VecOps;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

const LIGHT_BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &ObjectList, depth: usize) -> Color {
        if depth == 0 {
            return BLACK;
        }

        if let Some(record) = world.hit(self, Interval::new(0.0001, f64::MAX)) {
            let direction = record.normal + Vec3::random_unit_on_hemisphere(&record.normal);
            let new_ray = Ray {
                origin: record.point,
                direction,
            };

            return 0.5 * new_ray.color(world, depth - 1);
        }

        let unit_direction = self.direction.unit_vector();

        // get blend coefficient by having y coordinates between 0 and 1
        let blend_coef = 0.5 * (unit_direction.y + 1.0);

        (1.0 - blend_coef) * WHITE + blend_coef * LIGHT_BLUE
    }
}
