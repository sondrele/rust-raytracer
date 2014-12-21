use std::cell::Cell;

use vec::Vec3;

#[deriving(Clone)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
    vacuum: Cell<bool>
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            ori: Vec3::new(),
            dir: Vec3::new(),
            vacuum: Cell::new(true)
        }
    }

    pub fn init(ori: Vec3, dir: Vec3) -> Ray {
        let mut ray = Ray::new();
        ray.ori = ori;
        ray.dir = dir;
        ray
    }

    pub fn switch_medium(&self) {
        match self.vacuum.get() {
            true => self.vacuum.set(false),
            false => self.vacuum.set(true)
        }
    }

    pub fn in_vacuum(&self) -> bool {
        self.vacuum.get()
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;

    #[test]
    fn can_init_ray() {
        let r = Ray::init(Vec3::init(0.0, 1.0, 2.0), Vec3::init(2.0, 1.0, 0.0));
        assert_eq!(r.ori[2], 2.0);
        assert_eq!(r.dir[2], 0.0);
    }
}
