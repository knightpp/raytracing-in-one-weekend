use super::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new() -> Self {
        const aspect_ratio: f64 = 16.0 / 9.0;
        const viewport_height: f64 = 2.0;
        const viewport_width: f64 = aspect_ratio * viewport_height;
        const focal_length: f64 = 1.0;
        let origin = Point3::default();
        let horizontal = (viewport_width, 0., 0.).into();
        let vertical = (0., viewport_height, 0.).into();
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - (0., 0., focal_length).into();
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
