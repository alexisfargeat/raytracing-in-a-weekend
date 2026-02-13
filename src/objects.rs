use crate::ray::Ray;
use crate::vec3::Color;

pub mod sphere;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> bool;

    fn color(&self) -> Color;
}
