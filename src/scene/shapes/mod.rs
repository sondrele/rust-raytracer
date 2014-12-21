use std::mem::swap;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use self::Primitive::{Sphere, Poly};

pub mod sphere;
pub mod poly;

pub enum ShapeIntersection<'a> {
    Hit(f32),
    Missed
}

#[deriving(Copy)]
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

pub trait Shape {
    fn get_bbox(&self) -> BoundingBox;

    fn intersects(&self, ray: &Ray) -> ShapeIntersection;

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3;

    fn get_material(&self) -> Material;

    fn diffuse_color(&self, point: Vec3) -> Color;
}

pub enum Primitive {
    Poly(poly::Poly),
    Sphere(sphere::Sphere)
}

impl Shape for Primitive {
    fn get_bbox(&self) -> BoundingBox {
        match self {
            &Poly(ref poly) => poly.get_bbox(),
            &Sphere(ref sphere) => sphere.get_bbox(),
        }
    }

    fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        match self {
            &Poly(ref poly) => poly.intersects(ray),
            &Sphere(ref sphere) => sphere.intersects(ray),
        }
    }

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3 {
        match self {
            &Poly(ref poly) => poly.surface_normal(direction, point),
            &Sphere(ref sphere) => sphere.surface_normal(direction, point),
        }
    }

    fn get_material(&self) -> Material {
        match self {
            &Poly(ref poly) => poly.get_material(),
            &Sphere(ref sphere) => sphere.get_material(),
        }
    }

    fn diffuse_color(&self, point: Vec3) -> Color {
        match self {
            &Poly(ref poly) => poly.diffuse_color(point),
            &Sphere(_) => self.get_material().diffuse,
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::Shape;
    use scene::shapes::sphere::Sphere;

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

        assert!(bbox.intersects(ray));
    }
}
