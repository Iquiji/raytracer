use crate::hitable::{Hitable,hitable,HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::sphere::Sphere;
pub struct HitableList{
    pub hitable: Vec<hitable>,

}
impl Hitable for HitableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut rec : HitRecord = HitRecord{
            t: 0.0,
            p: Vec3::new(0.0,0.0,0.0),
            normal: Vec3::new(0.0,0.0,0.0),
        };
        let mut hit_anything: bool = false;
        let mut closest_so_far : f64 = t_max;
        for i in 0..self.hitable.len(){
            if let temp = self.hitable[i].hit(&r,t_min,closest_so_far).unwrap(){
                hit_anything = true;
                closest_so_far = temp.t;
                rec = temp;
            }
        }
        if hit_anything{
            return Some(rec);
        }
        return None;
    }
}