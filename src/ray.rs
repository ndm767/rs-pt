use crate::linalg::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }
}
