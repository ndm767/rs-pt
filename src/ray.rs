use crate::geometry::HitData;
use crate::geometry::Hittable;
use crate::geometry::Object;
use crate::linalg::Vec3;
use crate::scene::Scene;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
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

        Vec3::new(1.0, 0.0, 0.0)
    }
}
