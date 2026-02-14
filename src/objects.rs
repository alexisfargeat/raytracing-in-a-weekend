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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;

    fn normal(&self, point: Point3) -> Vec3;
}
