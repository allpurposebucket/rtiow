use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_float, vec3::{self, random_unit_vector}};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { 1.0 } else { fuzz }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(vec3::unit_vector(&ray_in.direction()), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz * vec3::random_unit_vector());

        if vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64, // index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = vec3::unit_vector(&ray_in.direction());
        let cos_theta = f64::min(vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
            vec3::reflect(unit_direction, hit_record.normal)
        } else {
            vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation: Color::with_values(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_record.p, direction),
        })
    }
}
