use vec::Vec3;
use ray::Ray;
use scene::bvh::Tree;
use scene::material::Color;
use scene::shapes::Shape;
use scene::intersection::Intersection;

pub mod material;
pub mod shapes;
pub mod intersection;
pub mod bvh;

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
    pub shapes: Vec<Box<Shape+'a>>,
    tree: Tree<'a>
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            camera: Camera::new(),
            lights: Vec::new(),
            shapes: Vec::new(),
            tree: Tree::new()
        }
    }

    pub fn build_tree(&'a mut self) {
        self.tree.init(self.shapes.as_mut_slice())
    }

    pub fn intersects(&'a self, ray: Ray) -> SceneIntersection<'a> {
        match self.tree.root {
            bvh::Empty => self.iterative_search(ray),
            _ => self.tree_search(ray)
        }
    }

    fn iterative_search(&'a self, ray: Ray) -> SceneIntersection<'a> {
        let mut intersection = Missed;
        let mut point: f32 = 0.0;

        let mut has_intersected = false;
        for shape in self.shapes.iter() {
            match shape.intersects(ray) {
                shapes::Hit(new_point) if !has_intersected => {
                    has_intersected = true;
                    point = new_point;
                    intersection = Intersected(Intersection::new(point, ray, shape));
                },
                shapes::Hit(new_point) if has_intersected && new_point < point => {
                    point = new_point;
                    intersection = Intersected(Intersection::new(point, ray, shape));
                },
                _ => ()
            }
        }
        intersection
    }

    fn tree_search(&'a self, ray: Ray) -> SceneIntersection<'a> {
        let intersection = self.tree.intersects(ray);
        match intersection {
            bvh::Hit(node, point) => Intersected(Intersection::new(point, ray, node.get_shape())),
            bvh::Missed => Missed
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::Scene;
    use scene::Intersected;
    use scene::shapes::sphere::Sphere;
    use scene::material::{Color, Material};

    fn create_scene<'a>() -> Scene<'a> {
        let mut sphere = Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        sphere.materials.insert(0, Material::init(Color::init(1.0, 0.0, 0.0)));
        let mut scene = Scene::new();
        scene.shapes.push(box sphere);
        scene
    }

    #[test]
    fn can_init_scene() {
        let scene = Scene::new();
        assert!(scene.lights.len() == 0);
        assert!(scene.shapes.len() == 0);
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
