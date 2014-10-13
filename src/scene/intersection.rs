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

    pub fn refractive_ray(&self) -> Option<Ray> {
        let in_dir = self.ray.dir;
        let mut normal = self.surface_normal();

        // Calculate medium index, only switching between air and glass for now
        let n: f32 = if self.ray.in_vacuum() {
            1.0 / 1.5
        } else {
            1.5 / 1.0
        };

        let cos_in = normal.dot(in_dir);
        if cos_in  > 0.0 {
           normal = normal.invert();
        }

        let c: f32 = in_dir.dot(normal);
        let cos_phi_2: f32 = 1.0 - n * n * (1.0 - c * c);
        if cos_phi_2 < 0.0 {
            None // Total internal reflection
        } else {
            let cos_phi: f32 = cos_phi_2.sqrt();
            let term1: Vec3 = in_dir - normal.mult(c);
            let term1: Vec3 = term1.mult(n);

            let direction: Vec3 = term1 - normal.mult(cos_phi);
            let origin = self.point() - normal.mult(0.01);
            let ray = Ray::init(origin, direction);
            ray.switch_medium();
            Some(ray)
        }
    }

    pub fn diffuse_color(&self) -> material::Color {
        self.shape.diffuse_color(self.point())
    }
}
