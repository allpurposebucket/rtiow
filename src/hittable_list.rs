use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(hit_record) =  object.hit(ray, Interval { min: interval.min, max: closest_so_far }) {
                closest_so_far = hit_record.t;
                temp_rec = Some(hit_record);
            }
        }

       temp_rec 
    }
}
