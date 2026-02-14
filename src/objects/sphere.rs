use crate::objects::HitRecord;
use crate::objects::Object;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::VecOps;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
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
            None
        } else {
            let candidate_t = (h - discriminant.sqrt()) / a;

            if candidate_t >= tmin && candidate_t <= tmax {
                let hit_point = ray.at(candidate_t);
                Some(HitRecord {
                    point: hit_point,
                    normal: self.normal(hit_point),
                    t: candidate_t,
                })
            } else {
                None
            }
        }
    }

    fn normal(&self, point: Point3) -> Vec3 {
        let vector_from_center_to_point = point - self.center;

        vector_from_center_to_point.unit_vector()
    }
}
