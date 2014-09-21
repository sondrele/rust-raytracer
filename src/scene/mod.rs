use vec::Vec3;
use ray::Ray;
use scene::material::Color;
use scene::shapes::ShapeType;
use scene::shapes::{Shape, Intersection, Intersected, Missed};

pub mod material;
pub mod shapes;

// Is it possible to mplement traits instead of using enums,
// make the fields private data and call trait methods
// according to the different types of lights?
#[deriving(PartialEq, Clone, Show)]
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
    pub view_dir: Vec3,
    pub focal_dist: f32,
    pub ortho_up: Vec3,
    pub vertical_fov: f32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Vec3::new(),
            view_dir: Vec3::new(),
            focal_dist: 0.0,
            ortho_up: Vec3::new(),
            vertical_fov: 0.0
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shapes: Vec<ShapeType>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            lights: Vec::new(),
            shapes: Vec::new()
        }
    }

    pub fn intersects(&self, ray: Ray) -> Intersection {
        for shape in self.shapes.iter() {
            match shape.intersects(ray) {
                Intersected(i) => return Intersected(i),
                Missed => ()
            }
        }
        Missed
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