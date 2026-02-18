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

        let cannot_refract = refraction_index * sin_theta > 1.0;

        let scattered_ray_direction =
            if cannot_refract || reflectance(cos_theta, refraction_index) > rand::random() {
                unit_direction.reflect(&normal)
            } else {
                unit_direction.refract(&normal, refraction_index)
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

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // uses Schlick's approximation
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
