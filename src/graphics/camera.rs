use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect: f64,
}
impl Camera {
    // pub fn std() -> Self {
    //     Self {
    //         origin: Vec3::new(0.0, 0.0, 0.0),
    //         lower_left_corner: Vec3::new(-2.0, -1.5, -1.0),
    //         horizontal: Vec3::new(4.0, 0.0, 0.0),
    //         vertical: Vec3::new(0.0, 3.0, 0.0),
    //     }
    // }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta: f64 = vfov * std::f64::consts::PI / 180.0;
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = (aspect) * half_height;
        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = vup.cross(w).unit_vector();
        let v: Vec3 = w.cross(u);
        Self {
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
            origin: lookfrom,
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect,
        }
    }
    pub fn mv(&self,mv_vec: Vec3,aspect: f64) -> Self{
        Self::new(self.lookfrom + mv_vec,self.lookat,self.vup,self.vfov,aspect)
    }
}
