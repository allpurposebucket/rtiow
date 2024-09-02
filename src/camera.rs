use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    color::{write_color, Color}, hittable::Hittable, interval::Interval, ray::{Point3, Ray}, utils::{degrees_to_radians, random_float}, vec3::{self, random_in_unit_disk, unit_vector, Vec3}
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pixel_samples_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(),
            look_at: Point3::with_values(0.0, 0.0, -1.0),
            vup: Vec3::with_values(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            pixel_samples_scale: 0.1,
            image_height: 100,
            center: Point3::with_values(0., 0., 0.),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(self.image_width, self.image_height);
        let bar = ProgressBar::new(self.image_width as u64 * self.image_height as u64 * self.samples_per_pixel as u64);

        img.enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                    bar.inc(1);
                }
                write_color(pixel, self.pixel_samples_scale * pixel_color);
        });

        bar.finish();

        img.save("output.jpg").unwrap();
    }

    fn initialize(&mut self) {
        let img_width_f32 = self.image_width as f32;

        let img_height_f32 = img_width_f32 / self.aspect_ratio;
        self.image_height = if img_height_f32 < 1.0 {
            1
        } else {
            img_height_f32 as u32
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height as f32 * (img_width_f32 / img_height_f32);

        self.w = unit_vector(&(self.look_from - self.look_at));
        self.u = unit_vector(&vec3::cross(&self.vup, &self.w));
        self.v = vec3::cross(&self.w, &self.u);

        let viewport_u = viewport_width as f64 * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
            - (self.focus_distance * self.w)
            - (viewport_u / 2.)
            - (viewport_v / 2.);

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_distance * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::with_values(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::with_values(random_float() - 0.5, random_float() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new();
        }

        if let Some(hit_record) = world.hit(ray, Interval::UNIVERSE) {
            if let Some(scatter_record) = hit_record.mat.as_ref().scatter(ray, &hit_record) {
                return scatter_record.attenuation * self.ray_color(&scatter_record.scattered, depth - 1, world);
            }

            return Color::new();
            // let direction = hit_record.normal + random_unit_vector();
            // return 0.5 * self.ray_color(&Ray::with_values(hit_record.p, direction), depth - 1, world);
        }

        let unit_direction = unit_vector(&ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0)
    }
}
