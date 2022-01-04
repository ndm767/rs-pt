use crate::linalg::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub pos: Vec3,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

impl Camera {
    pub fn new(pos: Vec3) -> Camera {
        Camera {
            pos,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn translate(&mut self, deltax: f64, deltay: f64, deltaz: f64) {
        self.pos.x += deltax;
        self.pos.y += deltay;
        self.pos.z += deltaz;
    }

    pub fn translate_in_dir(&mut self, forward: f64, right: f64, up: f64) {
        let new_dir = Vec3::new(right, up, forward).rotate(self.pitch, self.yaw, self.roll);

        self.translate(new_dir.x, new_dir.y, new_dir.z);
    }

    pub fn rotate(&mut self, delta_pitch: f64, delta_yaw: f64, delta_roll: f64) {
        self.pitch += delta_pitch;
        self.yaw += delta_yaw;
        self.roll += delta_roll;
    }

    pub fn gen_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let mut dir = Vec3::new(0.0, 0.0, 1.0);
        // random sampling within the pixel to smooth out edges
        let noise_x: f64 = rand::random();
        let noise_y: f64 = rand::random();
        dir.x = (x as f64 + noise_x) / ((width as f64) / 2.0) - 1.0;
        dir.y = (y as f64 + noise_y) / ((height as f64) / 2.0) - 1.0;
        dir = dir.rotate(self.pitch, self.yaw, self.roll);
        dir = dir.normalize();
        Ray {
            orig: self.pos,
            dir,
            depth: 0,
            weight: 1.0,
        }
    }
}
