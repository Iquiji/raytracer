#![allow(dead_code)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }
    pub fn len_sq(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.len();
        *self *= k;
    }
    pub fn dot(&self, f: Vec3) -> f64 {
        self.e[0] * f.e[0] + self.e[1] * f.e[1] + self.e[2] * f.e[2]
    }
    pub fn cross(&self, f: Vec3) -> Vec3 {
        Self::new(
            self.e[1] * f.e[2] - self.e[2] * f.e[1],
            -(self.e[0] * f.e[2] - self.e[2] * f.e[0]),
            self.e[0] * f.e[1] - self.e[1] * f.e[0],
        )
    }
    pub fn unit_vector(&self) -> Self {
        let f = self.len();
        Self::new(self.e[0] / f, self.e[1] / f, self.e[2] / f)
    }
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * 2.0 * Vec3::dot(self, *n)
    }
    pub fn refract(&self,n:Vec3,ni_over_nt: f64) -> Option<Vec3>{
        let uv : Vec3 = self.unit_vector();
        let dt :f64 = uv.dot(n);
        let discriminant : f64 = 1.0-ni_over_nt*ni_over_nt*(1.0-dt*dt);
        if discriminant > 0.0{
            return Some((uv-n*dt)*ni_over_nt-n*discriminant.sqrt());
        }
        None
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.e[0] *= f;
        self.e[1] *= f;
        self.e[2] *= f;
    }
}
impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, f: Vec3) {
        self.e[0] *= f.e[0];
        self.e[1] *= f.e[1];
        self.e[2] *= f.e[2];
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, f: Vec3) -> Self {
        Self::new(self.e[0] + f.e[0], self.e[1] + f.e[1], self.e[2] + f.e[2])
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, f: Vec3) -> Self {
        Self::new(self.e[0] - f.e[0], self.e[1] - f.e[1], self.e[2] - f.e[2])
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, f: Vec3) -> Self {
        Self::new(self.e[0] * f.e[0], self.e[1] * f.e[1], self.e[2] * f.e[2])
    }
}
impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, f: Vec3) -> Self {
        Self::new(self.e[0] / f.e[0], self.e[1] / f.e[1], self.e[2] / f.e[2])
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, f: f64) -> Self {
        Self::new(self.e[0] * f, self.e[1] * f, self.e[2] * f)
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, f: f64) -> Self {
        Self::new(self.e[0] / f, self.e[1] / f, self.e[2] / f)
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, f: Vec3) {
        self.e[0] += f.e[0];
        self.e[1] += f.e[1];
        self.e[2] += f.e[2];
    }
}
impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, f: Vec3) {
        self.e[0] -= f.e[0];
        self.e[1] -= f.e[1];
        self.e[2] -= f.e[2];
    }
}
impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, f: Vec3) {
        self.e[0] /= f.e[0];
        self.e[1] /= f.e[1];
        self.e[2] /= f.e[2];
    }
}
impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        self.e[0] /= f;
        self.e[1] /= f;
        self.e[2] /= f;
    }
}
