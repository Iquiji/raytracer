use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{hitableEnum, HitRecord, Hitable};
use crate::sphere::Sphere;
pub trait Material{
    fn scatter(&self,r: &Ray,rec: &HitRecord,attunuation: &mut Vec3) -> Option<Ray>;
}
#[derive(Debug,Clone,Copy)]
pub enum MaterialEnum{
    Metal(Metal),
    Lambertian(Lambertian),
}
impl Material for MaterialEnum{
    fn scatter(&self,r: &Ray,rec: &HitRecord,attunuation: &mut Vec3) -> Option<Ray>{
        use MaterialEnum::*;
        match self{
            Metal(m) => {
                return m.scatter(r,rec,attunuation);
            }
            Lambertian(m) => {
                return m.scatter(r,rec,attunuation);
            }
        }
    }
}


#[derive(Debug,Clone,Copy)]
pub struct Metal{
    albedo: Vec3,
}
impl Metal{
    pub fn new(r: f64,g: f64,b: f64) -> Self{
        Metal{
            albedo : Vec3::new(r, g, b),
        }
    }
}
impl Material for Metal{
    fn scatter(&self,r: &Ray,rec: &HitRecord,attunuation: &mut Vec3) -> Option<Ray>{
        let reflected : Vec3 = Vec3::reflect(&r.direction(), &rec.normal);
        let scattered : Ray = Ray::new(rec.p, reflected);
        *attunuation = self.albedo;
        if (Vec3::dot(&scattered.direction(),rec.normal) > 0.0){
            return Some(scattered);
        }
        return None;
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Lambertian{
    albedo: Vec3,
}
impl Material for Lambertian{
    fn scatter(&self,r: &Ray,rec: &HitRecord,attunuation: &mut Vec3) -> Option<Ray>{
        let target : Vec3 = rec.p + rec.normal + Sphere::random_in_unit_sphere();
        *attunuation = self.albedo;
        return Some(Ray::new(rec.p, target-rec.p));
    }
}
impl Lambertian{
    pub fn new(r: f64,g: f64,b: f64) -> Self{
        Lambertian{
            albedo : Vec3::new(r, g, b),
        }
    }
}