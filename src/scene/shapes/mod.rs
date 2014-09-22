use ray::Ray;
use scene::material::Color;
use scene::shapes::sphere::Sphere;
use scene::shapes::polyset::PolySet;

pub mod sphere;
pub mod poly;
pub mod polyset;

#[deriving(PartialEq, Show)]
pub enum Intersection {
    Intersected(f32),
    IntersectedWithColor(f32, Color),
    Missed
}

pub enum ShapeType {
    SphereType(Sphere),
    PolySetType(PolySet)
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> Intersection;
}

impl Shape for ShapeType {
    fn intersects(&self, ray: Ray) -> Intersection {
        match self {
            &SphereType(ref sphere) => sphere.intersects(ray),
            &PolySetType(ref polyset) => polyset.intersects(ray)
        }
    }
}
