use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        assert!(fuzz <= 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.reflect(&hit_record.normal) + self.fuzz * Vec3::random_unit();
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
