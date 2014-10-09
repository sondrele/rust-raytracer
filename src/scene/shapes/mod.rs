use vec::Vec3;
use ray::Ray;
use scene::material::Material;

pub mod sphere;
pub mod poly;
pub mod polyset;

pub enum Intersection<'a> {
    Intersected(f32),
    Missed
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> Intersection;
    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3;
    fn get_material(&self) -> Material;
}
