use std::ops::{Add, Div, Mul, Sub};

use std::u32;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Operator overloading for Vec3 with Vec3.
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// Operator overloading for Vec3 with f32.
impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // You would only use this over the other length for extra performance.
    pub fn length_square(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    // This will make it so that the length is one.
    pub fn normalize(self) -> Vec3 {
        let l: f32 = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self / l
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: -(v1.x * v2.z - v1.z * v2.x),
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub struct RandomSeries {
    pub state: u32,
}

impl RandomSeries {
    pub fn random_bilateral(&mut self) -> f32 {
        xor_shift32(self) as f32 / u32::MAX as f32
    }

    pub fn rand_in_unit_sphere(&mut self) -> Vec3 {
        let mut p: Vec3;
        p = 2.0 * Vec3::new(
            self.random_bilateral(),
            self.random_bilateral(),
            self.random_bilateral(),
        ) - Vec3::new(1.0, 1.0, 1.0);

        while p.length_square() >= 1.0 {
            p = 2.0 * Vec3::new(
                self.random_bilateral(),
                self.random_bilateral(),
                self.random_bilateral(),
            ) - Vec3::new(1.0, 1.0, 1.0);
        }

        p
    }
}

fn xor_shift32(rs: &mut RandomSeries) -> u32 {
    let mut x: u32 = rs.state;

    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;

    rs.state = x;

    x
}
