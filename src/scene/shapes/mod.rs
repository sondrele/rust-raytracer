// use vec::Vec3;
use ray::Ray;
use scene::material::{Color, Material};
pub mod sphere;
pub mod poly;
pub mod polyset;

pub enum Intersection<'a> {
    Intersected(f32),
    IntersectedWithColor(f32, Color),
    IntersectedWithIndex(f32, uint),
    Missed
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> Intersection;
    // fn surface_normal(&self, point: Vec3) -> Vec3;
    fn get_material(&self) -> Material;
}
