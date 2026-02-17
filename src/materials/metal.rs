use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.reflect(&hit_record.normal);
        let scattered_ray = Ray {
            direction: reflected,
            origin: hit_record.point,
        };

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo,
        })
    }
}
