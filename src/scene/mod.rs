use vec::Vec3;
use scene::material::Color;
use scene::shapes::Shape;

pub mod material;
pub mod shapes;

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

pub struct Camera {
    pub pos: Vec3,
    pub viewDir: Vec3,
    pub focalDist: f32,
    pub orthoUp: Vec3,
    pub verticalFOV: f32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Vec3::new(),
            viewDir: Vec3::new(),
            focalDist: 0.0,
            orthoUp: Vec3::new(),
            verticalFOV: 0.0
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shapes: Vec<Shape>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            lights: Vec::new(),
            shapes: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use scene::Scene;

    #[test]
    fn can_init_scene() {
        let scene = Scene::new();
        assert!(scene.lights.len() == 0);
        assert!(scene.shapes.len() == 0);
    }
}