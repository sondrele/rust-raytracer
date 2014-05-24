use vec::Vec3;
use material::Color;
use shapes::Sphere;

// Is it possible to mplement traits instead of using enums,
// make the fields private data and call trait methods
// according to the different types of lights?
#[deriving(Eq, Clone, Show)]
pub enum LightType {
    PointLight,
    AreaLight,
    DirectionalLight
}

pub struct Light {
    pub kind: LightType,
    pub pos: Vec3,
    pub dir: Vec3,
    pub intensity: Color
}

impl Light {
    pub fn new(kind: LightType) -> Light {
        Light{ kind: kind, pos: Vec3::new(), dir: Vec3::new(), intensity: Color::new() }
    }
}

pub struct RScene {
    lights: Vec<Light>,
    shapes: Vec<Sphere>
}

impl RScene {
    pub fn new() -> RScene {
        RScene{ lights: Vec::new(), shapes: Vec::new() }
    }
}

#[test]
fn can_init_scene() {
    let scene = RScene::new();
    assert!(scene.lights.len() == 0);
    assert!(scene.shapes.len() == 0);
}
