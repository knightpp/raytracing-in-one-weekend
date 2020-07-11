use super::*;
use std::sync::Arc;

pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    
    pub material : Option<Arc<dyn Material + Send + Sync>>,
}
impl HitRecord {
    pub fn default() -> Self{
        HitRecord{
            p : Default::default(),
            normal : Default::default(),
            t : Default::default(),
            front_face : Default::default(),
            material : None,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl<T: Hittable> Hittable for Vec<T>{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hr = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.iter().rev(){
            if obj.hit(ray, t_min, t_max, &mut hr){
                hit_anything = true;
                closest_so_far = hr.t;
                //*rec = hr.clone();
            }
        }
        *rec = hr;
        hit_anything
    }
}