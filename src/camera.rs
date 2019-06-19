use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn std() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.5, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 3.0, 0.0),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
    pub fn new(vfov : f64,aspect : f64) -> Self {
        let theta: f64 = vfov*std::f64::consts::PI/180.0;
        let half_height : f64 = (theta/2.0).tan();
        let half_width : f64 = aspect*half_height;
        Self{
            lower_left_corner: Vec3::new(-half_width,-half_height,-1.0),
            horizontal: Vec3::new(2.0*half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0*half_height, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        } 
    }
}
