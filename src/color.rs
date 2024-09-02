use image::Rgb;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }
    0.0
}

pub fn write_color(pixel: &mut Rgb<u8>, pixel_color: Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256. * intensity.clamp(r)) as u8;
    let gbyte = (256. * intensity.clamp(g)) as u8;
    let bbyte = (256. * intensity.clamp(b)) as u8;

    *pixel = Rgb([rbyte, gbyte, bbyte]);
}
