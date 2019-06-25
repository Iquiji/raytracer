use crate::hitable::{HitRecord, Hitable};
use crate::material::MaterialEnum;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: MaterialEnum,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: MaterialEnum) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
                - Vec3::new(1.0, 1.0, 1.0);
            if p.len_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}
impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().dot(r.direction());
        let b: f64 = oc.dot(r.direction());
        let c: f64 = oc.dot(oc) - (self.radius * self.radius);
        let discriminant: f64 = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.mat,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.mat,
                });
            }
        }
        None
    }
}
