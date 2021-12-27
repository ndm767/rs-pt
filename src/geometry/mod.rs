pub mod sphere;

use crate::linalg::Vec3;
use crate::ray::Ray;
pub use sphere::Sphere;

#[allow(dead_code)]
pub enum Object {
    Sphere(Sphere),
}

// data structure to hold the values returned from a ray hit
#[allow(dead_code)]
pub struct HitData {
    hit_pos: Vec3,
    hit_norm: Vec3,
    coef: f64,
}

impl HitData {
    pub fn new(hit_pos: Vec3, hit_norm: Vec3, coef: f64) -> HitData {
        HitData {
            hit_pos,
            hit_norm,
            coef,
        }
    }
}

pub trait Hittable {
    fn calc_intersection(&self, _ray: Ray) -> Option<HitData> {
        None
    }
}

impl Hittable for Object {
    fn calc_intersection(&self, _ray: Ray) -> Option<HitData> {
        None
    }
}