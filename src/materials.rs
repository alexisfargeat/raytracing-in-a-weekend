use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}
