use crate::objects::Object;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3, VecOps};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        // return true if the following polynomial admit at least a root
        // P(X) = d.d X^2 - 2 d.(C - Q) X + (C - Q) . (C - Q) - r^2
        // where Ray(t) = Q + t * d, C (resp. r) is the center (resp. radius) of the sphere,
        // and . is the dot product

        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&(self.center - ray.origin));
        let c =
            (self.center - ray.origin).dot(&(self.center - ray.origin)) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (h - discriminant.sqrt()) / a
        }
    }

    fn color(&self, ray: &Ray) -> Color {
        let intersection_point = self.intersect(ray);
        assert!(intersection_point >= 0.0);

        let normal_vector = self.normal(ray.at(intersection_point));

        0.5 * Color {
            x: normal_vector.x + 1.0,
            y: normal_vector.y + 1.0,
            z: normal_vector.z + 1.0,
        }
    }

    fn normal(&self, point: Point3) -> Vec3 {
        let vector_from_center_to_point = point - self.center;

        vector_from_center_to_point.unit_vector()
    }
}
