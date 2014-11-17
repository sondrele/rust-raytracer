use vec::Vec3;
use ray::Ray;
use scene::material::Color;
use scene::shapes::{PolyPrim, SpherePrim, Shape};
use scene::intersection::Intersection;

pub mod material;
pub mod shapes;
pub mod intersection;

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
        Light{
            kind: kind,
            pos: Vec3::new(),
            dir: Vec3::new(),
            intensity: Color::new()
        }
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

pub enum SceneIntersection<'a> {
    Intersected(Intersection<'a>),
    Missed
}

pub struct Scene<'a> {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub primitives: Vec<shapes::Primitive>
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            camera: Camera::new(),
            lights: Vec::new(),
            primitives: Vec::new()
        }
    }

    pub fn intersects(&'a self, ray: Ray) -> SceneIntersection {
        let mut intersection = Missed;
        let mut point: f32 = 0.0;

        let mut has_intersected = false;

        // TODO: Duplicate code
        for prim in self.primitives.iter() {
            match prim {
                &PolyPrim(ref poly) => match poly.intersects(ray) {
                    shapes::Hit(new_point) if !has_intersected => {
                        has_intersected = true;
                        point = new_point;
                        intersection = Intersected(Intersection::new(point, ray, prim));
                    },
                    shapes::Hit(new_point) if has_intersected && new_point < point => {
                        point = new_point;
                        intersection = Intersected(Intersection::new(point, ray, prim));
                    },
                    _ => ()
                },
                &SpherePrim(ref sphere) => match sphere.intersects(ray) {
                    shapes::Hit(new_point) if !has_intersected => {
                        has_intersected = true;
                        point = new_point;
                        intersection = Intersected(Intersection::new(point, ray, prim));
                    },
                    shapes::Hit(new_point) if has_intersected && new_point < point => {
                        point = new_point;
                        intersection = Intersected(Intersection::new(point, ray, prim));
                    },
                    _ => ()
                }
            }
        }

        intersection
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::{Scene, Intersected};
    use scene::shapes::SpherePrim;
    use scene::shapes::sphere::Sphere;
    use scene::material::{Color, Material};

    fn create_scene<'a>() -> Scene<'a> {
        let mut sphere = Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        sphere.materials.insert(0, Material::init(Color::init(1.0, 0.0, 0.0)));
        let mut scene = Scene::new();
        scene.primitives.push(SpherePrim(sphere));
        scene
    }

    #[test]
    fn can_init_scene() {
        let scene = Scene::new();
        assert!(scene.lights.len() == 0);
        assert!(scene.primitives.len() == 0);
    }

    #[test]
    fn can_intersect_scene() {
        let scene = create_scene();

        match scene.intersects(Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0))) {
            Intersected(intersection) => {
                let color = intersection.color();
                assert_eq!(Color::init(1.0, 0.0, 0.0), color)
            },
            _ => fail!("Ray did not intersect scene")
        }
    }
}
