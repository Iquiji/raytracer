use crate::vec3::Vec3;
pub struct Ray{
    origin: Vec3,
    direction: Vec3,
}
impl Ray{
    pub fn new(a: Vec3,b: Vec3) -> Self{
        Ray{origin: a,direction: b}
    }
    pub fn origin(&self) -> Vec3{
        self.origin
    }
    pub fn direction(&self) -> Vec3{
        self.direction
    }
    pub fn point_at_parameter(&self,t: f64) -> Vec3{
        self.origin + self.direction*t
    }
}