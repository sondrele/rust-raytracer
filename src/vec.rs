use std::cmp::Ordering;
use std::num::Float;
use std::ops::{Add, Sub, Mul, Index};

#[derive(Clone, Copy, Show)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PartialEq for Vec3 {
    fn eq(&self, vec: &Vec3) -> bool {
        self.x == vec.x && self.y == vec.y && self.z == vec.z
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<Ordering> {
        match self < other {
            true => Some(Ordering::Less),
            false => Some(Ordering::Greater)
        }
    }

    fn lt(&self, vec: &Vec3) -> bool {
        if self[0] > vec[0] {
            false
        } else if self[0] < vec[0] {
            true
        } else if self[1] < vec[1] {
            true
        } else if self[1] == vec[1] && self[2] < vec[2] {
            true
        } else {
            false
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3::init(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3::init(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::init(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }
}

impl Index<u32> for Vec3 {
    type Output = f32;

    fn index<'a>(&'a self, index: &u32) -> &'a f32 {
        match index {
            &0 => &self.x,
            &1 => &self.y,
            &2 => &self.z,
            _ => panic!("Index out of bounds: {}", index)
        }
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::init(0.0, 0.0, 0.0)
    }

    pub fn init(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn mult(&self, num: f32) -> Vec3 {
        Vec3::init(self.x * num, self.y * num, self.z * num)
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len != 0.0 {
            self.x = self.x / len;
            self.y = self.y / len;
            self.z = self.z / len;
        }
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;
        Vec3::init(x, y, z)
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn invert(&self) -> Vec3 {
        self.mult(-1.0)
    }

    pub fn distance(&self, other: Vec3) -> f32 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        let c = self.z - other.z;
        (a * a + b * b + c * c).sqrt()
    }

    pub fn get_area(a: Vec3, b: Vec3, c: Vec3) -> f32 {
        let ab = b - a;
        let ac = c - a;
        ab.cross(ac).length() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;

    #[test]
    fn vec3_can_be_addded(){
        let a = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let b = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let c = a + b;
        assert_eq!(c.x, 0f32);
        assert_eq!(c.y, 2f32);
        assert_eq!(c.z, 4f32);
    }

    #[test]
    fn vec3_can_be_subtracted(){
        let a = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let b = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let c = a - b;
        assert_eq!(c.x, 0f32);
        assert_eq!(c.y, 0f32);
        assert_eq!(c.z, 0f32);
    }

    #[test]
    fn vec3_can_be_multiplied(){
        let a = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let b = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let c = a * b;
        assert_eq!(c.x, 0f32);
        assert_eq!(c.y, 1f32);
        assert_eq!(c.z, 4f32);
    }

    #[test]
    fn vec3_can_be_multiplied_with_f32(){
        let a = Vec3{x: 0f32, y: 1f32, z: 2f32};
        let c = a.mult(2.0);
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, 4.0);
    }

    #[test]
    fn vec3_can_be_equal(){
        let a = Vec3{x: 1.2, y: 2.2, z: 3.2};
        let b = Vec3{x: 1.2, y: 2.2, z: 3.2};

        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
        assert_eq!(a.z, b.z);
    }

    #[test]
    fn vec3_has_length(){
        let a = Vec3{x: 1.2, y: 2.2, z: 3.2};
        let x = a.length();
        assert!(x-4.06448 > 0.0);
        assert!(x-4.06449 < 0.0);
    }

    #[test]
    fn vec3_can_be_normalized(){
        let mut v = Vec3::init(3.0, 4.0, 5.0);
        v.normalize();
        assert!(v.x-0.424264 > 0.0);
        assert!(v.x-0.424265 < 0.0);
    }

    #[test]
    fn vec3_has_crossproduct(){
        let x = Vec3::init(1.0, 2.0, 3.0);
        let y = Vec3::init(3.0, 4.0, 5.0);

        let z = x.cross(y);
        assert_eq!(z.x, -2.0);
        assert_eq!(z.y, 4.0);
        assert_eq!(z.z, -2.0);
    }

    #[test]
    fn vec3_can_be_indexed(){
        let x = Vec3::init(1.0, 2.0, 3.0);
        assert_eq!(x[0], 1.0);
        assert_eq!(x[1], 2.0);
        assert_eq!(x[2], 3.0);
    }
}
