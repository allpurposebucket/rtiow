use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_float, random_float_range};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    components: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            components: [0.0, 0.0, 0.0],
        }
    }

    pub fn with_values(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            components: [x, y, z],
        }
    }

    pub fn x(&self) -> f64 {
        self.components[0]
    }

    pub fn y(&self) -> f64 {
        self.components[1]
    }

    pub fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.components[0] * self.components[0]
            + self.components[1] * self.components[1]
            + self.components[2] * self.components[2]
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.components[0].abs() < EPS && self.components[1].abs() < EPS && self.components[2].abs() < EPS
    }

    pub fn random() -> Vec3 {
        Vec3::with_values(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::with_values(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.components[0], self.components[1], self.components[2]
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            components: [
                self.components[0] + rhs.components[0],
                self.components[1] + rhs.components[1],
                self.components[2] + rhs.components[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            components: [
                self.components[0] - rhs.components[0],
                self.components[1] - rhs.components[1],
                self.components[2] - rhs.components[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            components: [
                self.components[0] * rhs.components[0],
                self.components[1] * rhs.components[1],
                self.components[2] * rhs.components[2],
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            components: [
                self.components[0] * rhs,
                self.components[1] * rhs,
                self.components[2] * rhs,
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            components: [
                self * rhs.components[0],
                self * rhs.components[1],
                self * rhs.components[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            components: [
                self.components[0] / rhs,
                self.components[1] / rhs,
                self.components[2] / rhs,
            ],
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            components: [
                self.components[0] / rhs,
                self.components[1] / rhs,
                self.components[2] / rhs,
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            components: [
                -self.components[0],
                -self.components[1],
                -self.components[2],
            ],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.components[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.components[0] += rhs.components[0];
        self.components[1] += rhs.components[1];
        self.components[2] += rhs.components[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.components[0] *= rhs;
        self.components[1] *= rhs;
        self.components[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.components[0] /= rhs;
        self.components[1] /= rhs;
        self.components[2] /= rhs;
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        //  Sadly, we have a small floating-point abstraction leak to deal with.
        //  Since floating-point numbers have finite precision, a very small value can underflow to zero when squared.
        //  So if all three coordinates are small enough (that is, very near the center of the sphere),
        //  the norm of the vector will be zero, and thus normalizing will yield the bogus vector [±∞,±∞,±∞]
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / f64::sqrt(lensq);
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&(-uv), &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt((1.0 - r_out_perp.length_squared()).abs()) * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::with_values(random_float_range(-1.0, 1.0), random_float_range(-1.0, 1.0), 0.);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.components[0] * v.components[0]
        + u.components[1] * v.components[1]
        + u.components[2] * v.components[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        components: [
            u.components[1] * v.components[2] - u.components[2] * v.components[1],
            u.components[2] * v.components[0] - u.components[0] * v.components[2],
            u.components[0] * v.components[1] - u.components[1] * v.components[0],
        ],
    }
}
