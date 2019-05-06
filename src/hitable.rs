use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::sphere::Sphere;
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub enum hitable {
    Sphere,
}
impl Hitable for hitable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        match self{
            hitable::Sphere=> {
                return self.hit(&r, t_min, t_max);
            }
            _=> {
                println!("smh. is not implemented for enum hitable");
                return None;
            }
        }
    }
}