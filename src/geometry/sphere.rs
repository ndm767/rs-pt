use super::HitData;
use super::Hittable;
use crate::linalg::Vec3;
use crate::materials::Material;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn calc_intersection(&self, ray: &Ray) -> Option<HitData> {
        let dist: Vec3 = ray.orig - self.center;
        let a: f64 = ray.dir.dot(ray.dir);
        let b: f64 = 2.0 * ray.dir.dot(dist);
        let c: f64 = dist.dot(dist) - self.radius * self.radius;

        let disc: f64 = b * b - 4.0 * a * c;
        let mut t0: f64;
        let mut t1: f64;
        if disc < 0.0 {
            return None;
        } else if disc == 0.0 {
            t0 = -0.5 * b / a;
            t1 = t0;
        } else {
            let q: f64;
            if b > 0.0 {
                q = -0.5 * (b + disc.sqrt());
            } else {
                q = -0.5 * (b - disc.sqrt());
            }

            t0 = q / a;
            t1 = c / q;
        }

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        const EPSILON: f64 = 1e-8;
        if t0 < EPSILON {
            return None;
        }

        let hit_pos: Vec3 = ray.orig + ray.dir * t0;
        let hit_norm: Vec3 = (hit_pos - self.center).normalize();
        Some(HitData::new(hit_pos, hit_norm, t0, ray.depth + 1, self.mat))
    }
}
