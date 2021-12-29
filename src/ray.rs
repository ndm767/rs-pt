use crate::geometry::HitData;
use crate::geometry::Hittable;
use crate::geometry::Object;
use crate::linalg::Vec3;
use crate::materials::Brdf;
use crate::materials::Material;
use crate::scene::Scene;
use rand::thread_rng;
use rand::Rng;

const MAX_DEPTH: i32 = 3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub depth: i32,
    pub weight: f64,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3, depth: i32, weight: f64) -> Ray {
        Ray {
            orig,
            dir,
            depth,
            weight,
        }
    }

    pub fn get_color(self, hit_data: HitData, scene: &Scene) -> Vec3 {
        if self.depth < MAX_DEPTH {
            let mut rng = thread_rng();
            let prob: f32 = rng.gen();
            if prob < 0.5 {
                match hit_data.mat {
                    Material::Lambertian(l) => return l.scatter(hit_data, scene, self.weight),
                    Material::PerfectSpecular(p) => return p.scatter(hit_data, scene, self.weight),
                }
            } else {
                match hit_data.mat {
                    Material::Lambertian(l) => return l.emission * self.weight,
                    Material::PerfectSpecular(p) => return p.emission * self.weight,
                }
            }
        }

        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn trace(self, scene: &Scene) -> Vec3 {
        let mut hit_data: Option<HitData> = None;
        for o in &scene.objs {
            let temp_hit_data: Option<HitData>;
            match o {
                Object::Sphere(s) => {
                    temp_hit_data = s.calc_intersection(&self);
                }
            }

            // see if the hit is closer than any previous hit
            if let Some(thd_value) = temp_hit_data {
                if let Some(hd_value) = hit_data {
                    if hd_value.coef > thd_value.coef {
                        hit_data = temp_hit_data;
                    }
                } else {
                    hit_data = temp_hit_data;
                }
            }
        }

        if hit_data.is_none() {
            return scene.ambient;
        }

        self.get_color(hit_data.unwrap(), scene)
    }
}
