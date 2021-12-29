use crate::linalg::Vec3;
use rand::prelude::*;

pub fn random_unit_vector() -> Vec3 {
    let mut x: f64 = 10.0;
    let mut y: f64 = 10.0;
    let mut z: f64 = 10.0;

    let mut rng = rand::thread_rng();
    while (x * x + y * y + z * z) >= 1.0 {
        x = rng.gen();
        x = x * 2.0 - 1.0;
        y = rng.gen();
        y = y * 2.0 - 1.0;
        z = rng.gen();
        z = z * 2.0 - 1.0;
    }

    Vec3::new(x, y, z).normalize()
}
