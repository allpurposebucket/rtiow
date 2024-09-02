use std::sync::Arc;

use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, ray::{Point3, Ray}, vec3::{dot, Vec3}};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(&ray.direction(), &oc);
        let c = oc.length_squared() - (self.radius*self.radius);

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);
        
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            t: root,
            p: ray.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };

        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Sphere { 
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}
