use crate::consts::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn length_squared(&self) -> f32 {
        // return self.x * self.x + self.y * self.y + self.z * self.z;
        return dot(*self, *self);
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(self.length_squared());
    }

    pub fn unit_vector(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn random() -> Self {
        Self {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        return unit_vector(Vec3::random_in_unit_sphere());
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, *normal) > 0.0 {
            return in_unit_sphere;
        }

        return -in_unit_sphere;
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return f32::abs(self.x) < s && f32::abs(self.y) < s && f32::abs(self.z) < s;
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(dot(-uv, n), 1.0);

    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;

    return r_out_perp + r_out_parallel;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * dot(v, n) * n;
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self: Vec3, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self - v.x,
            y: self - v.y,
            z: self - v.z,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self / v.x,
            y: self / v.y,
            z: self / v.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self + v.x,
            y: self + v.y,
            z: self + v.z,
        }
    }
}
