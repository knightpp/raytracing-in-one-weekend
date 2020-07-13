use super::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hr = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.iter() {
            if obj.hit(ray, t_min, closest_so_far, &mut hr) {
                hit_anything = true;
                closest_so_far = hr.t;
            }
        }
        *rec = hr;
        hit_anything
    }
}
