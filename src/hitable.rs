use crate::material::MaterialEnum;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialEnum,
}
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
#[derive(Debug)]
pub enum HitableEnum {
    SphereE(Sphere),
}
impl Hitable for HitableEnum {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HitableEnum::SphereE(sphere) => {
                sphere.hit(&r, t_min, t_max)
            }
        }
    }
}
