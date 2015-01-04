use std::rand::{random, Open01};
use std::num::Float;
use std::num::FloatMath;

use vec::Vec3;
use ray::Ray;
use scene::bvh::{NodeIntersection, Tree};
use scene::material::Color;
use scene::shapes::{Shape, ShapeIntersection};
use scene::intersection::Intersection;
use self::SceneIntersection::{Intersected, Missed};
use self::Light::{Point, Area, Directional};

pub mod parser;
pub mod from_obj;
pub mod material;
pub mod shapes;
pub mod intersection;
pub mod bvh;

#[derive(Copy, PartialEq, Clone, Show)]
pub enum Light {
    Point(PointLight),
    Area(AreaLight),
    Directional(DirectionalLight)
}

impl Light {
    pub fn intensity(&self) -> Color {
        match self {
            &Point(ref light) => light.intensity,
            &Area(ref light) => light.intensity,
            &Directional(ref light) => light.intensity
        }
    }

    pub fn position(&self) -> Vec3 {
        match self {
            &Point(ref light) => light.pos,
            &Area(ref light) => light.sample_point(),
            &Directional(_) => Vec3::new()
        }
    }

    pub fn get_dir(&self, point: Vec3) -> Vec3 {
        match self {
            &Light::Directional(ref light) => {
                light.dir.invert()
            },
            &Light::Point(ref light) => {
                let mut dir = light.pos - point;
                dir.normalize();
                dir
            },
            &Light::Area(ref light) => {
                let mut dir = light.sample_point() - point;
                dir.normalize();
                dir
            }
        }
    }
}

#[derive(Copy, PartialEq, Clone, Show)]
pub struct PointLight {
    pub pos: Vec3,
    pub intensity: Color
}

impl PointLight {
    pub fn new() -> PointLight {
        PointLight {
            pos: Vec3::new(),
            intensity: Color::new()
        }
    }
}

#[derive(Copy, PartialEq, Clone, Show)]
pub struct AreaLight {
    pub min: Vec3,
    pub max: Vec3,
    pub intensity: Color
}

impl AreaLight {
    pub fn new() -> AreaLight {
        AreaLight {
            min: Vec3::new(),
            max: Vec3::new(),
            intensity: Color::new()
        }
    }

    pub fn sample_point(&self) -> Vec3 {
        let Open01(rx) = random::<Open01<f32>>();
        let Open01(ry) = random::<Open01<f32>>();
        let Open01(rz) = random::<Open01<f32>>();
        let mut dx = (self.max[0] - self.min[0]).abs() * 0.5;
        let mut dy = (self.max[1] - self.min[1]).abs() * 0.5;
        let mut dz = (self.max[2] - self.min[2]).abs() * 0.5;
        dx = dx - rx * (dx * 2.0);
        dy = dy - ry * (dy * 2.0);
        dz = dz - rz * (dz * 2.0);
        Vec3::init(self.max[0] + dx, self.max[1] + dy, self.max[2] + dz)
    }
}

#[derive(Copy, PartialEq, Clone, Show)]
pub struct DirectionalLight {
    pub dir: Vec3,
    pub intensity: Color
}

impl DirectionalLight {
    pub fn new() -> DirectionalLight {
        DirectionalLight {
            dir: Vec3::new(),
            intensity: Color::new()
        }
    }
}

#[derive(Copy)]
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

pub trait IntersectableScene<'a> {
    fn get_camera(&self) -> &Camera;

    fn get_lights(&self) -> &[Light];

    fn intersects(&'a self, ray: &Ray) -> SceneIntersection<'a>;
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
}

impl<'a> IntersectableScene<'a> for Scene<'a> {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }

    fn get_lights(&self) -> &[Light] {
        self.lights.as_slice()
    }

    fn intersects(&'a self, ray: &Ray) -> SceneIntersection<'a> {
        let mut intersection = Missed;
        let mut point: f32 = 0.0;

        let mut has_intersected = false;
        for prim in self.primitives.iter() {
            match prim.intersects(ray) {
                ShapeIntersection::Hit(new_point) if !has_intersected => {
                    has_intersected = true;
                    point = new_point;
                    intersection = Intersected(Intersection::new(point, ray.clone(), prim));
                },
                ShapeIntersection::Hit(new_point) if has_intersected && new_point < point => {
                    point = new_point;
                    intersection = Intersected(Intersection::new(point, ray.clone(), prim));
                },
                _ => ()
            }
        }
        intersection
    }
}

pub struct BvhScene<'a> {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub tree: Tree<'a>
}

impl<'a> BvhScene<'a> {
    pub fn new() -> BvhScene<'a> {
        BvhScene {
            camera: Camera::new(),
            lights: Vec::new(),
            tree: Tree::new()
        }
    }

    pub fn from_scene(scene: Scene<'a>) -> BvhScene<'a> {
        let mut bvh_scene = BvhScene::new();
        bvh_scene.camera = scene.camera;
        bvh_scene.lights = scene.lights;
        bvh_scene.tree.init(scene.primitives);
        bvh_scene
    }
}

impl<'a> IntersectableScene<'a> for BvhScene<'a> {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }

    fn get_lights(&self) -> &[Light] {
        self.lights.as_slice()
    }

    fn intersects(&'a self, ray: &Ray) -> SceneIntersection<'a> {
        let intersection = self.tree.intersects(ray);
        match intersection {
            NodeIntersection::Hit(node, point) =>
                Intersected(Intersection::new(point, ray.clone(), node.get_shape())),
            NodeIntersection::Missed => Missed
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::{IntersectableScene, Scene, SceneIntersection};
    use scene::shapes::{sphere, Primitive};
    use scene::material::{Color, Material};

    fn create_scene<'a>() -> Scene<'a> {
        let mut sphere = sphere::Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        sphere.materials.insert(0, Material::init(Color::init(1.0, 0.0, 0.0)));
        let mut scene = Scene::new();
        scene.primitives.push(Primitive::Sphere(sphere));
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

        match scene.intersects(&Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0))) {
            SceneIntersection::Intersected(intersection) => {
                let color = intersection.color();
                assert_eq!(Color::init(1.0, 0.0, 0.0), color)
            },
            _ => panic!("Ray did not intersect scene")
        }
    }
}
