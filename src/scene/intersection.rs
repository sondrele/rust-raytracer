use ray::Ray;
use vec::Vec3;
use scene::shapes::Shape;
use scene::material;

pub struct Intersection<'a> {
    point: f32,
    ray: Ray,
    shape: &'a Box<Shape+'a>
}

impl<'a> Intersection<'a> {
    pub fn new(point: f32, ray: Ray, shape: &'a Box<Shape>) -> Intersection<'a> {
        Intersection {
            point: point,
            ray: ray,
            shape: shape
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.ray.dir
    }

    pub fn point(&self) -> Vec3 {
        self.ray.ori + self.ray.dir.mult(self.point)
    }

    pub fn color(&self) -> material::Color {
        self.shape.get_material().diffuse
    }

    pub fn material(&self) -> material::Material {
        self.shape.get_material()
    }

    pub fn surface_normal(&self) -> Vec3 {
        self.shape.surface_normal(self.ray.dir, self.point())
    }

    pub fn reflective_ray(&self) -> Ray {
        let normal = self.surface_normal();
        let d0 = self.ray.dir.invert();
        let origin = self.point() + normal.mult(0.0001);
        let direction = normal.mult(d0.dot(normal) * 2.0) - d0;
        Ray::init(origin, direction)
    }
}
