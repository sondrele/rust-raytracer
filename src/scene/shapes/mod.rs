use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};

pub mod sphere;
pub mod poly;
pub mod polyset;

pub enum ShapeIntersection<'a> {
    Hit(f32),
    Missed
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> ShapeIntersection;
    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3;
    fn get_material(&self) -> Material;
    fn diffuse_color(&self, _: Vec3) -> Color {
      self.get_material().diffuse
    }
}
