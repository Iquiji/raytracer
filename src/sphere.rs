use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: Vec3, rd: f64) -> Self {
        Self {
            center: center,
            radius: rd,
        }
    }
}
impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().dot(r.direction());
        let b: f64 = oc.dot(r.direction()) * 2.0;
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if (temp < t_max && temp > t_min) {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                });
            }
        }
        return None;
    }
}
