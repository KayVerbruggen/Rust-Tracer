use math::Vec3;
use ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    llc: Vec3,
    hor: Vec3,
    vert: Vec3,
    origin: Vec3
}

pub fn new() -> Camera {
    Camera {
        llc: Vec3::new(-2.0, -1.0, -1.0),
        hor: Vec3::new(4.0, 0.0, 0.0),
        vert: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        ray::new(self.origin, self.llc + u*self.hor + v*self.vert - self.origin)
    } 
}