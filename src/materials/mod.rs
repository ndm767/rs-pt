pub mod lambertian;
pub mod perfect_specular;
pub mod util;

use crate::geometry::HitData;
use crate::linalg::Vec3;
use crate::scene::Scene;

pub use lambertian::Lambertian;
pub use perfect_specular::PerfectSpecular;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    PerfectSpecular(PerfectSpecular),
}

pub trait Brdf {
    fn scatter(&self, _hit_data: HitData, _scene: &Scene, _prev_weight: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Brdf for Material {
    fn scatter(&self, _hit_data: HitData, _scene: &Scene, _prev_weight: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
