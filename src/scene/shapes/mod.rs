use std::mem::swap;
use std::cmp::Ordering;
use std::num::FloatMath;
use std::ops::Add;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use self::Primitive::{MeshPoly, Poly, Sphere};

pub mod sphere;
pub mod poly;
pub mod poly_mesh;

pub enum ShapeIntersection<'a> {
    Hit(f32),
    Missed
}

#[derive(Copy, PartialEq, Show)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox {
            min: Vec3::new(),
            max: Vec3::new()
        }
    }

    pub fn init(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox {
            min: min,
            max: max
        }
    }

    pub fn centroid(&self) -> Vec3 {
        self.min.mult(0.5) + self.max.mult(0.5)
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let ori = ray.ori;
        let dir = ray.dir;

        let mut tmin = (self.min[0] - ori[0]) / dir[0];
        let mut tmax = (self.max[0] - ori[0]) / dir[0];
        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min[1] - ori[1]) / dir[1];
        let mut tymax = (self.max[1] - ori[1]) / dir[1];
        if tymin > tymax {
            swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min[2] - ori[2]) / dir[2];
        let mut tzmax = (self.max[2] - ori[2]) / dir[2];
        if tzmin > tzmax {
            swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        true
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, other: BoundingBox) -> BoundingBox {
        let min = Vec3::init(
            self.min[0].min(other.min[0]),
            self.min[1].min(other.min[1]),
            self.min[2].min(other.min[2])
        );

        let max = Vec3::init(
            self.max[0].max(other.max[0]),
            self.max[1].max(other.max[1]),
            self.max[2].max(other.max[2])
        );

        BoundingBox::init(min, max)
    }
}

impl PartialOrd for BoundingBox {
    fn partial_cmp(&self, other: &BoundingBox) -> Option<Ordering> {
        match self < other {
            true => Some(Ordering::Less),
            false => Some(Ordering::Greater)
        }
    }

    fn lt(&self, bbox: &BoundingBox) -> bool {
        self.centroid() < bbox.centroid()
    }
}

pub trait Shape {
    fn get_bbox(&self) -> BoundingBox;

    fn intersects(&self, ray: &Ray) -> ShapeIntersection;

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3;

    fn get_material(&self) -> Material;

    fn diffuse_color(&self, point: Vec3) -> Color;
}

#[derive(Clone, PartialEq, Show)]
pub enum Primitive {
    MeshPoly(poly_mesh::Poly),
    Poly(poly::Poly),
    Sphere(sphere::Sphere)
}

impl Shape for Primitive {
    fn get_bbox(&self) -> BoundingBox {
        match self {
            &MeshPoly(ref poly) => poly.get_bbox(),
            &Poly(ref poly) => poly.get_bbox(),
            &Sphere(ref sphere) => sphere.get_bbox()
        }
    }

    fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        match self {
            &MeshPoly(ref poly) => poly.intersects(ray),
            &Poly(ref poly) => poly.intersects(ray),
            &Sphere(ref sphere) => sphere.intersects(ray)
        }
    }

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3 {
        match self {
            &MeshPoly(ref poly) => poly.surface_normal(direction, point),
            &Poly(ref poly) => poly.surface_normal(direction, point),
            &Sphere(ref sphere) => sphere.surface_normal(direction, point)
        }
    }

    fn get_material(&self) -> Material {
        match self {
            &MeshPoly(ref poly) => poly.get_material(),
            &Poly(ref poly) => poly.get_material(),
            &Sphere(ref sphere) => sphere.get_material()

        }
    }

    fn diffuse_color(&self, point: Vec3) -> Color {
        match self {
            &MeshPoly(ref poly) => poly.diffuse_color(point),
            &Poly(ref poly) => poly.diffuse_color(point),
            &Sphere(_) => self.get_material().diffuse
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::Shape;
    use scene::shapes::sphere::Sphere;
    use scene::shapes::BoundingBox;

    #[test]
    fn can_create_boundingbox_from_sphere() {
        let s = Sphere::init(Vec3::init(1.0, 1.0, 1.0), 2.0);
        let bbox = s.get_bbox();
        assert_eq!(Vec3::init(-1.0, -1.0, -1.0), bbox.min);
        assert_eq!(Vec3::init(3.0, 3.0, 3.0), bbox.max);
    }

    #[test]
    fn can_intersect_bbox() {
        let s = Sphere::init(Vec3::init(1.0, 1.0, 1.0), 2.0);
        let bbox = s.get_bbox();
        let ray = Ray::init(Vec3::init(0.0, 0.0, -2.0), Vec3::init(0.0, 0.0, -1.0));

        assert!(bbox.intersects(&ray));
    }

    #[test]
    fn can_compare_bbox_based_on_centroid() {
        let b0 = BoundingBox::init(Vec3::init(-1.0, 0.0, 0.0), Vec3::init(0.0, 1.0, 1.0));
        let b1 = BoundingBox::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 1.0, 1.0));

        assert!(b0 < b1);
    }
}
