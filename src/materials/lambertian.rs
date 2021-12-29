use super::util::random_unit_vector;
use super::Brdf;
use crate::geometry::HitData;
use crate::linalg::Vec3;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub emission: Vec3,
    pub diffuse: Vec3,
    pub reflectance: f64,
}

impl Lambertian {
    pub fn new(emission: Vec3, diffuse: Vec3, reflectance: f64) -> Lambertian {
        Lambertian {
            emission,
            diffuse,
            reflectance,
        }
    }
}

impl Brdf for Lambertian {
    fn scatter(&self, hit_data: HitData, scene: &Scene, prev_weight: f64) -> Vec3 {
        let mut new_dir = hit_data.hit_norm + random_unit_vector();

        const SMALL: f64 = 1e-8;
        if new_dir.x.abs() < SMALL && new_dir.y.abs() < SMALL && new_dir.z.abs() < SMALL {
            new_dir = hit_data.hit_norm;
        }

        let new_ray = Ray::new(
            hit_data.hit_pos,
            new_dir,
            hit_data.depth + 1,
            prev_weight * self.reflectance,
        );

        new_ray.trace(scene)
    }
}
