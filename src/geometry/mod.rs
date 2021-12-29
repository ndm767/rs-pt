pub mod sphere;

use crate::linalg::Vec3;
use crate::materials::Material;
use crate::ray::Ray;
pub use sphere::Sphere;

pub enum Object {
    Sphere(Sphere),
}

// data structure to hold the values returned from a ray hit
#[derive(Copy, Clone)]
pub struct HitData {
    pub hit_pos: Vec3,
    pub hit_norm: Vec3,
    pub coef: f64,
    pub depth: i32,
    pub mat: Material,
}

impl HitData {
    pub fn new(hit_pos: Vec3, hit_norm: Vec3, coef: f64, depth: i32, mat: Material) -> HitData {
        HitData {
            hit_pos,
            hit_norm,
            coef,
            depth,
            mat,
        }
    }
}

pub trait Hittable {
    fn calc_intersection(&self, _ray: &Ray) -> Option<HitData> {
        None
    }
}

impl Hittable for Object {
    fn calc_intersection(&self, _ray: &Ray) -> Option<HitData> {
        None
    }
}
