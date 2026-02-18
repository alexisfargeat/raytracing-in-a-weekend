use crate::materials::Material;
use crate::materials::Scatter;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::VecOps;

pub struct Dielectric {
    refraction_coef: f64,
    attenuation: Color,
}

impl Dielectric {
    pub fn new(refraction_coef: f64, attenuation: Color) -> Self {
        Self {
            refraction_coef,
            attenuation,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let (refraction_index, normal) = if hit_record.normal.dot(&ray.direction) > 0.0 {
            (self.refraction_coef, -1.0 * hit_record.normal)
        } else {
            (1.0 / self.refraction_coef, hit_record.normal)
        };

        let unit_direction = ray.direction.unit_vector();

        let cos_theta = -unit_direction.dot(&normal);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let scattered_ray_direction = if refraction_index * sin_theta <= 1.0 {
            unit_direction.refract(&normal, refraction_index)
        } else {
            unit_direction.reflect(&normal)
        };

        // let scattered_ray_direction = unit_direction.refract(&normal, refraction_index);

        Some(Scatter {
            ray: Ray {
                origin: hit_record.point,
                direction: scattered_ray_direction,
            },
            attenuation: self.attenuation,
        })
    }
}
