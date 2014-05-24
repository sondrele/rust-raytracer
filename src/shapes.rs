use material::Material;
use vec::Vec3;

// struct Triangle {
//     materials: Vec<Material>,

// }

pub struct Sphere {
    materials: Vec<Material>,
    origin: Vec3,
    radius: f32,
    xaxis: Vec3,
    xlength: f32,
    yaxis: Vec3,
    ylength: f32,
    zaxis: Vec3,
    zlength: f32
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere{
            materials: Vec::new(),
            origin: Vec3::new(),
            radius: 0.0,
            xaxis: Vec3::new(),
            xlength: 0.0,
            yaxis: Vec3::new(),
            ylength: 0.0,
            zaxis: Vec3::new(),
            zlength: 0.0
        }
    }
}

#[test]
fn sphere_can_initializes(){
    let s = Sphere::new();
    assert!(s.radius == 0.0);
}