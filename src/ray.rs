use math;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: math::Vec3,
    pub direction: math::Vec3
}

pub fn new(o: math::Vec3, dir: math::Vec3) -> Ray {
    Ray {
        origin: o,
        direction: dir
    }
}

impl Ray {
    pub fn point_at(self, t: f32) -> math::Vec3 {
        self.origin + t*self.direction
    }
}