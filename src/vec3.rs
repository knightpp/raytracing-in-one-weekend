use super::*;
use std::marker::PhantomData;
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub x: f64,
    pub y: f64,
    pub z: f64,

    marker: PhantomData<T>,
}
impl<T> std::ops::AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T> std::ops::AddAssign<f64> for Vec3<T> {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
impl<T> std::ops::SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<T> std::ops::SubAssign<f64> for Vec3<T> {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
impl<T> std::ops::MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl<T> std::ops::MulAssign<f64> for Vec3<T> {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<T> std::ops::DivAssign<Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl<T> std::ops::DivAssign<f64> for Vec3<T> {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl<T> std::ops::Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            marker: PhantomData,
        }
    }
}

impl<T> std::ops::Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            marker: PhantomData,
        }
    }
}
impl<T> std::ops::Sub for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            marker: PhantomData,
        }
    }
}
impl<T> std::ops::Div for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            marker: PhantomData,
        }
    }
}
impl<T> std::ops::Mul for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            marker: PhantomData,
        }
    }
}

impl<T> std::ops::Div<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn div(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs /= self;
        rhs
    }
}
impl<T> std::ops::Div<f64> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}
impl<T> std::ops::Mul<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn mul(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl<T> From<(f64, f64, f64)> for Vec3<T> {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Vec3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            marker: PhantomData,
        }
    }
}
impl<T> From<(i32, i32, i32)> for Vec3<T> {
    fn from(t: (i32, i32, i32)) -> Self {
        Vec3 {
            x: t.0 as f64,
            y: t.1 as f64,
            z: t.2 as f64,
            marker: PhantomData,
        }
    }
}

impl<T> Vec3<T>
where
    T: ExpressibleInThree,
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x,
            y,
            z,
            marker: PhantomData,
        }
    }
    pub fn zeroed() -> Vec3<T> {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
            marker: PhantomData,
        }
    }
    pub fn dot(&self, v: &Vec3<T>) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
    pub fn cross(&self, v: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
            marker: PhantomData,
        }
    }
    pub fn unit(&self) -> Vec3<T> {
        let len = self.len();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            marker: PhantomData,
        }
    }
    pub fn len(&self) -> f64 {
        self.len_sqrt().sqrt()
    }

    pub fn len_sqrt(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn random() -> Self {
        let a: f64 = random();
        let z: f64 = random();
        let r: f64 = (1. - z * z).sqrt();
        Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z,
            marker: PhantomData,
        }
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Vec3 {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max),
            marker: PhantomData,
        }
    }
    pub fn random_in_unit_sphere() -> Vec3<T> {
        loop {
            let p = Vec3::random();
            if p.len_sqrt() >= 1.0 {
            } else {
                return p;
            }
        }
    }
    pub fn random_in_hemisphere(normal: &Vec3<T>) -> Vec3<T> {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn reflect(&self, n: &Vec3<T>) -> Vec3<T> {
        (*self) - 2.0 * self.dot(n) * (*n)
    }
    pub fn refract(&self, n: &Vec3<T>, etai_over_etat: f64) -> Vec3<T> {
        let cos_theta = (-(*self)).dot(n);
        let r_out_parallel = etai_over_etat * ((*self) + cos_theta * (*n));
        let r_out_perp = -(1.0 - r_out_parallel.len_sqrt()).sqrt() * (*n);
        r_out_parallel + r_out_perp
    }
}

impl Vec3<Color>{
    pub fn as_point3(self) -> Vec3<Point3>{
        Vec3{
            marker: PhantomData,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl Vec3<Point3>{
    pub fn as_color(self) -> Vec3<Color>{
        Vec3{
            marker: PhantomData,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub trait ExpressibleInThree: Copy {}

#[derive(Debug, Default, Copy, Clone)]
pub struct Color();
impl ExpressibleInThree for Color {}
#[derive(Debug, Default, Copy, Clone)]
pub struct Point3();
impl ExpressibleInThree for Point3 {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dot() {
        let v = Vec3::new(-6., 8., 0.);
        let normal = Vec3::new(5., 12., 0.);
        let product = v.dot(&normal);
        assert_eq!(product, 66.0);
    }

    #[test]
    fn refract() {
        let perp = Vec3::new(1., 1., 0.);
        let normal = Vec3::new(1., 0., 0.);
        let res = perp.refract(&normal, 1.5);
        eprintln!("{:?}", res);
    }
}
