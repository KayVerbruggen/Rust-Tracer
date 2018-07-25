use material;
use math;
use math::Vec3;
use ray;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Box<material::Material>,
}

pub trait Hitable: Send {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Box<material::Material>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, m: Box<material::Material>) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            mat: m,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a: f32 = math::dot(r.direction, r.direction);
        let b: f32 = math::dot(oc, r.direction);
        let c: f32 = math::dot(oc, oc) - self.radius * self.radius;

        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - discriminant.sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at(temp),
                    normal: (r.point_at(temp) - self.center) / self.radius,
                    mat: self.mat.clone(),
                });
            }

            temp = (-b + discriminant.sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at(temp),
                    normal: (r.point_at(temp) - self.center) / self.radius,
                    mat: self.mat.clone(),
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

impl Hitable for Vec<Sphere> {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_any = false;
        let mut closest = t_max;
        let mut hr = HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: material::Diffuse::new(0.0, 0.0, 0.0),
        };

        for obj in self.iter() {
            let temp_hr = obj.hit(r, t_min, closest);
            match temp_hr {
                Some(rec) => {
                    hit_any = true;
                    closest = rec.t;
                    hr = rec
                }

                None => {}
            }
        }

        if hit_any {
            Some(hr)
        } else {
            None
        }
    }
}
