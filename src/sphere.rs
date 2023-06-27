use crate::hittable::{HitRecord, Hittable};
use crate::material::*;
use crate::math::vec3::{dot, Vec3};
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat: Some(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);

        if let Some(mat) = &self.mat {
            record.mat = Some(mat.clone());
        }

        record.normal = (record.point - self.center) / self.radius;

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
