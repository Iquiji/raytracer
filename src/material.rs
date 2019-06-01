use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray>;
}
#[derive(Debug, Clone, Copy)]
pub enum MaterialEnum {
    Metal(Metal),
    Lambertian(Lambertian),
}
impl Material for MaterialEnum {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray> {
        use MaterialEnum::*;
        match self {
            Metal(m) => m.scatter(r, rec, attunuation),
            Lambertian(m) => m.scatter(r, rec, attunuation),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Metal {
            albedo: Vec3::new(r, g, b),
            fuzz: if fuzz - 1.0 < 0.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray> {
        let reflected: Vec3 = Vec3::reflect(&r.direction().unit_vector(), &rec.normal);
        let scattered: Ray = Ray::new(
            rec.p,
            reflected + Sphere::random_in_unit_sphere() * self.fuzz,
        );
        *attunuation = self.albedo;
        if Vec3::dot(&scattered.direction(), rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray> {
        let target: Vec3 = rec.p + rec.normal + Sphere::random_in_unit_sphere();
        *attunuation = self.albedo;
        Some(Ray::new(rec.p, target - rec.p))
    }
}
impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Lambertian {
            albedo: Vec3::new(r, g, b),
        }
    }
}
