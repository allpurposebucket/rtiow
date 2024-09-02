use std::sync::Arc;

use crate::{camera::Camera, color::Color, hittable_list::HittableList, material::{Dielectric, Lambertian, Metal}, ray::Point3, sphere::Sphere, utils::{random_float, random_float_range}, vec3::Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

const OBJS_RANGE: i32 = 22;

fn main() {
    let material_ground = Arc::new(Lambertian::new(Color::with_values(0.5, 0.5, 0.5)));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, -1000., 0.0),
        1000.0,
        material_ground,
    )));

    for obj_a in -OBJS_RANGE/2..OBJS_RANGE/2 {
        for obj_b in -OBJS_RANGE/2..OBJS_RANGE/2 {
            let choose_mat = random_float();
            let center = Point3::with_values(obj_a as f64 + 0.9*random_float(), 0.2, obj_b as f64 + 0.9*random_float());
            if (center - Point3::with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }

        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::with_values(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::with_values(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::with_values(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::with_values(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));


    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.look_from = Point3::with_values(13.0, 2.0, 3.0);
    cam.look_at = Point3::with_values(0.0, 0.0, 0.0);
    cam.vup = Vec3::with_values(0.0, 1.0, 0.0);
    cam.vfov = 20.0;
    cam.defocus_angle = 0.6;
    cam.focus_distance = 10.0;

    cam.render(&world);

    println!("Image saved as output.jpg");
}
