use crate::materials::Material;
use crate::materials::Scatter;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &crate::ray::Ray,
        hit_record: &crate::objects::HitRecord,
    ) -> Option<Scatter> {
        let mut scatter_direction =
            hit_record.normal + Vec3::random_unit_on_hemisphere(&hit_record.normal);

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        };

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo,
        })
    }
}
