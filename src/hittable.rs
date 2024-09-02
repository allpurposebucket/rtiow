use std::sync::Arc;

use crate::{interval::Interval, material::Material, ray::{Point3, Ray}, vec3::{dot, Vec3}};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
