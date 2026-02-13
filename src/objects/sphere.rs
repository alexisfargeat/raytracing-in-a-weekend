use crate::objects::Object;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, VecOps};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // return true if the following polynomial admit at least a root
        // P(X) = d.d X^2 + 2 d.(C - Q) X + (C - Q) . (C - Q) - r^2
        // where Ray(t) = Q + t * d, C (resp. r) is the center (resp. radius) of the sphere,
        // and . is the dot product

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&(self.center - ray.origin));
        let c =
            (self.center - ray.origin).dot(&(self.center - ray.origin)) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }

    fn color(&self) -> Color {
        Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
