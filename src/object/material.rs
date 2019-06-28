use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::object::hitable::HitRecord;
use crate::object::sphere::Sphere;
use rand::prelude::*;
pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray>;
}
#[derive(Debug, Clone, Copy)]
pub enum MaterialEnum {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}
impl Material for MaterialEnum {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray> {
        use MaterialEnum::*;
        match self {
            Metal(m) => m.scatter(r, rec, attunuation),
            Lambertian(m) => m.scatter(r, rec, attunuation),
            Dielectric(m) => m.scatter(r, rec, attunuation),
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
#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ref_idx: f64,
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attunuation: &mut Vec3) -> Option<Ray> {
        let outward_normal: Vec3;
        let reflected: Vec3 = r_in.direction().reflect(&rec.normal);
        let ni_over_nt: f64;
        let reflect_prob: f64;
        let cosine: f64;
        *attunuation = Vec3::new(1.0, 1.0, 1.0);
        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.len();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(rec.normal) / r_in.direction.len();
        }
        let refracted = r_in.direction.refract(outward_normal, ni_over_nt);
        if refracted.is_some() {
            let r0: f64 = ((1.0 - self.ref_idx) / (1.0 + self.ref_idx))
                * ((1.0 - self.ref_idx) / (1.0 + self.ref_idx));
            reflect_prob = r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
        } else {
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            Some(Ray::new(rec.p, reflected))
        } else {
            Some(Ray::new(rec.p, refracted.unwrap()))
        }
    }
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}
