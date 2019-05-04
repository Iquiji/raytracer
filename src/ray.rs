use crate::vec3::Vec3;
pub struct Ray{
    origin: Vec3,
    direction: Vec3,
}
impl Ray{
    fn new(a: Vec3,b: Vec3) -> Self{
        Ray{origin: a,direction: b}
    }
    fn origin(&self) -> Vec3{
        self.origin
    }
    fn direction(&self) -> Vec3{
        self.direction
    }
    fn point_at_parameter(&self,t: f64) -> Vec3{
        self.origin + self.direction*t
    }
}