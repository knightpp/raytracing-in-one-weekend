use crate::Point3;
#[derive(Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Point3 {
        &self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn at() {
        let ray = Ray::new(Point3::new(1., 2., 3.), Point3::new(4., 5., 6.));
        eprintln!("{:?}", ray.at(1.0));
    }
}
