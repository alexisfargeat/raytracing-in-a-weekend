use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub mod sphere;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Object {
    /// Compute the hit point (if any) between the Object and a Ray between t_min and t_max
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;

    fn normal(&self, point: Point3) -> Vec3;
}

#[derive(Default)]
pub struct ObjectList<'a> {
    objects: Vec<&'a dyn Object>,
}

impl<'a> ObjectList<'a> {
    pub fn add(&mut self, object: &'a dyn Object) {
        self.objects.push(object);
    }

    /// Compute the first hit Object (if any) on the path of a Ray between t_min and t_max
    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut result_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;

        for obj in &self.objects {
            let hit_point = obj.hit(ray, ray_tmin, ray_tmax);

            if let Some(record) = hit_point
                && record.t <= closest_so_far
            {
                closest_so_far = record.t;
                result_record = Some(record);
            }
        }

        result_record
    }
}
