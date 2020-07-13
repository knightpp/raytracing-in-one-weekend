use crate::{material::Material, ray::Ray, Point3};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,

    pub material: Option<Arc<dyn Material + Send + Sync>>,
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            material: self.material.as_ref().map(|x| x.clone()),
            ..*self
        }
    }
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Point3::default(),
            t: Default::default(),
            front_face: Default::default(),
            material: None,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Point3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
