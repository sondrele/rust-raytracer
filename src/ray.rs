use vec::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            ori: Vec3::new(),
            dir: Vec3::new()
        }
    }

    pub fn init(ori: Vec3, dir: Vec3) -> Ray {
        Ray {
            ori: ori,
            dir: dir
        }
    }
}

#[test]
fn can_init_ray() {
    let r = Ray::init(Vec3::init(0.0, 1.0, 2.0), Vec3::init(2.0, 1.0, 0.0));
    assert_eq!(r.ori[2], 2.0);
    assert_eq!(r.dir[2], 0.0);
}