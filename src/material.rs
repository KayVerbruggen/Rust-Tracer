use math;
use math::Vec3;
use object;
use ray;

pub trait Material: MaterialClone + Send {
        fn scatter(
                &self,
                r: &ray::Ray,
                hr: &object::HitRecord,
                rng: &mut math::RandomSeries,
        ) -> (Vec3, ray::Ray, bool);
}

// To be honest I have absolutly no clue why and how this works.
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait MaterialClone {
        fn clone_box(&self) -> Box<Material>;
}

impl<T> MaterialClone for T
where
        T: 'static + Material + Clone,
{
        fn clone_box(&self) -> Box<Material> {
                Box::new(self.clone())
        }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Material> {
        fn clone(&self) -> Box<Material> {
                self.clone_box()
        }
}

#[derive(Copy, Clone)]
pub struct Diffuse {
        albedo: Vec3,
}

unsafe impl Send for Diffuse {}

impl Diffuse {
        pub fn new(r: f32, g: f32, b: f32) -> Box<Material> {
                Box::new(Diffuse {
                        albedo: Vec3::new(r, g, b),
                })
        }
}

impl Material for Diffuse {
        fn scatter(
                &self,
                _r: &ray::Ray,
                hr: &object::HitRecord,
                rng: &mut math::RandomSeries,
        ) -> (Vec3, ray::Ray, bool) {
                let target = hr.p + hr.normal + rng.rand_in_unit_sphere();

                (self.albedo, ray::new(hr.p, target - hr.p), true)
        }
}

#[derive(Copy, Clone)]
pub struct Metal {
        albedo: Vec3,
        fuzz: f32,
}

impl Metal {
        pub fn new(c: Vec3, f: f32) -> Box<Material> {
                if f >= 1.0 {
                        Box::new(Metal {
                                albedo: c,
                                fuzz: 1.0,
                        })
                } else {
                        Box::new(Metal { albedo: c, fuzz: f })
                }
        }
}

impl Material for Metal {
        fn scatter(
                &self,
                r: &ray::Ray,
                hr: &object::HitRecord,
                rng: &mut math::RandomSeries,
        ) -> (Vec3, ray::Ray, bool) {
                let reflected = reflect(r.direction.normalize(), hr.normal);
                let scattered: ray::Ray;

                // Saves a couple of random generations for what it's worth.
                if self.fuzz == 0.0 {
                        scattered = ray::new(hr.p, reflected);
                } else {
                        scattered =
                                ray::new(hr.p, reflected + self.fuzz * rng.rand_in_unit_sphere());
                }

                (
                        self.albedo,
                        scattered,
                        math::dot(scattered.direction, hr.normal) > 0.0,
                )
        }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * math::dot(v, n) * n
}
