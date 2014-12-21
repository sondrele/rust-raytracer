use vec::Vec3;
use ray::Ray;
use scene::bvh::{Node, NodeIntersection, Tree};
use scene::material::Color;
use scene::shapes::{Shape, ShapeIntersection};
use scene::intersection::Intersection;
use self::SceneIntersection::{Intersected, Missed};


pub mod parser;
pub mod material;
pub mod shapes;
pub mod intersection;
pub mod bvh;

// Is it possible to mplement traits instead of using enums,
// make the fields private data and call trait methods
// according to the different types of lights?
#[deriving(Copy, PartialEq, Clone, Show)]
pub enum LightType {
    PointLight,
    AreaLight,
    DirectionalLight
}

#[deriving(Copy)]
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

#[deriving(Copy)]
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

    pub fn intersects(&'a self, ray: &Ray) -> SceneIntersection<'a> {
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
    use scene::{Scene, SceneIntersection};
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
