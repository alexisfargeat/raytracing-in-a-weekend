use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub mod sphere;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> f64;

    fn color(&self, ray: &Ray) -> Color;

    fn normal(&self, point: Point3) -> Vec3;
}
