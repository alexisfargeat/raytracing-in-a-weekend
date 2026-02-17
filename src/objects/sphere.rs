use crate::materials::Material;
use crate::objects::HitRecord;
use crate::objects::Object;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::VecOps;

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Object<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
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
            return None;
        }

        let min_root = (h - discriminant.sqrt()) / a;
        let max_root = (h + discriminant.sqrt()) / a;
        if ray_t.contains(min_root) {
            let hit_point = ray.at(min_root);
            Some(HitRecord::new(
                hit_point,
                self.normal(hit_point),
                self.material,
                min_root,
            ))
        } else if ray_t.contains(max_root) {
            let hit_point = ray.at(max_root);
            Some(HitRecord::new(
                hit_point,
                self.normal(hit_point),
                self.material,
                max_root,
            ))
        } else {
            None
        }
    }

    fn normal(&self, point: Point3) -> Vec3 {
        let vector_from_center_to_point = point - self.center;

        vector_from_center_to_point.unit_vector()
    }
}
