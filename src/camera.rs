use super::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    u: Point3,
    v: Point3,
    _w: Point3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Point3,
        vfov: Degrees,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta: f64 = vfov.to_radians().into();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            _w: w,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd: Point3 = self.lens_radius * Point3::random_in_unit_disk();
        let offset: Point3 = (rd.x * self.u) + rd.y * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

pub struct Degrees(f64);
impl Degrees {
    pub fn new(val: f64) -> Self {
        Degrees(val)
    }
    fn to_radians(&self) -> Radians {
        Radians(self.0 * std::f64::consts::PI / 180.0)
    }
}
impl Into<f64> for Degrees {
    fn into(self) -> f64 {
        self.0
    }
}
pub struct Radians(f64);
impl Radians {
    fn to_degrees(&self) -> Radians {
        todo!()
    }
}
impl Into<f64> for Radians {
    fn into(self) -> f64 {
        self.0
    }
}
