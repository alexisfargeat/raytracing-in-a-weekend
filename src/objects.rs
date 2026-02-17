use crate::materials::Material;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub mod sphere;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
}

impl<'a> HitRecord<'a> {
    pub const fn new(point: Point3, normal: Vec3, material: &'a dyn Material, t: f64) -> Self {
        HitRecord {
            point,
            normal,
            material,
            t,
        }
    }
}

pub trait Object<'a> {
    /// Compute the hit point (if any) between the Object and a Ray between t_min and t_max
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<'a>>;

    fn normal(&self, point: Point3) -> Vec3;
}

#[derive(Default)]
pub struct ObjectList<'a> {
    objects: Vec<Box<dyn Object<'a> + 'a>>,
}

impl<'a> ObjectList<'a> {
    pub fn add(&mut self, object: impl Object<'a> + 'a) {
        self.objects.push(Box::new(object));
    }

    /// Compute the first hit Object (if any) on the path of a Ray between t_min and t_max
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<'a>> {
        let mut result_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max();

        for obj in &self.objects {
            let hit_point = obj.hit(ray, Interval::new(ray_t.min(), closest_so_far));

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
