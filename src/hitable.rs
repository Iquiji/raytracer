use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
#[derive(Debug,Clone,Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub enum hitableEnum {
    SphereE(Sphere),
}
impl Hitable for hitableEnum {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            SphereE => {
                return self.hit(&r, t_min, t_max);
            }
            _ => {
                println!("smh. is not implemented for enum hitable");
                return None;
            }
        }
    }
}
