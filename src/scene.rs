use crate::geometry::Object;
use crate::linalg::Vec3;

pub struct Scene {
    pub objs: Vec<Object>,
    pub ambient: Vec3,
}

impl Scene {
    pub fn new(ambient: Vec3) -> Scene {
        Scene {
            objs: vec![],
            ambient,
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objs.push(obj);
    }
}
