use super::*;
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hr = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.iter() {
            if obj.hit(ray, t_min, t_max, &mut hr) {
                hit_anything = true;
                closest_so_far = hr.t;
                //*rec = hr.clone();
                *rec = hr.clone();
            }
        }

        hit_anything
    }
}
