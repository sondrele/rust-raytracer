use std::num::FloatMath;
use std::ops::Index;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes::{BoundingBox, Shape, ShapeIntersection};

type RefVec<'a> = &'a Vec3;

// Index into the 'vertices' Vec in PolyMesh
pub type VertexIndex<'a> = &'a Vec3;

// Index into the 'normals' Vec in PolyMesh
pub type NormalIndex<'a> = &'a Vec3;

// Index into the 'materials' Vec in PolyMesh
pub type MaterialIndex<'a> = &'a Material;

pub type PolyVertex<'a> = (VertexIndex<'a>, Option<NormalIndex<'a>>, Option<MaterialIndex<'a>>);

pub struct PolyMesh<'a> {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub materials: Vec<Material>,
    pub polys: Vec<Poly<'a>>
}

impl<'a> PolyMesh<'a> {
    pub fn new() -> PolyMesh<'a> {
        PolyMesh {
            materials: Vec::new(),
            vertices: Vec::new(),
            normals: Vec::new(),
            polys: Vec::new()
        }
    }
}

pub struct Poly<'a> {
    pub x: PolyVertex<'a>,
    pub y: PolyVertex<'a>,
    pub z: PolyVertex<'a>
}

impl<'a> Index<u32> for Poly<'a> {
    type Output = Vec3;

    fn index<'b>(&'b self, index: &u32) -> &'b Vec3 {
        match index {
            &0 => self.x.0,
            &1 => self.y.0,
            &2 => self.z.0,
            _ => panic!("Index out of bound: {}", index)
        }
    }
}

impl<'a> Poly<'a> {
    pub fn new(x: &'a Vec3, y: &'a Vec3, z: &'a Vec3) -> Poly<'a> {
        Poly {
            x: (x, None, None),
            y: (y, None, None),
            z: (z, None, None)
        }
    }

    fn weighted_areas(&self, point: Vec3) -> (f32, f32, f32) {
        let area = Vec3::get_area(self[0], self[1], self[2]);
        let area0 = Vec3::get_area(self[0], self[1], point) / area;
        let area1 = Vec3::get_area(self[2], self[0], point) / area;
        let area2 = Vec3::get_area(self[1], self[2], point) / area;

        if area0 > 1.0 || area1 > 1.0 || area2 > 1.0 {
            panic!("Cannot get area, as point is outside of poly")
        }

        (area0, area1, area2)
    }

    fn interpolated_color(&self, point: Vec3) -> Color {
        match (self.x.2, self.y.2, self.z.2) {
            (Some(idx_x), Some(idx_y), Some(idx_z)) => {
                let (area0, area1, area2) = self.weighted_areas(point);
                idx_x.diffuse.mult(area2) + idx_y.diffuse.mult(area1)
                    + idx_z.diffuse.mult(area0)
            },
            _ => panic!("Not enough pointers to materials")
        }
    }

    fn static_normal(&self) -> Vec3 {
        let v = self[1] - self[0];
        let w = self[2] - self[0];
        let mut norm = v.cross(w);
        norm.normalize();
        norm
    }

    fn interpolated_normal(&self, point: Vec3) -> Vec3 {
        match (self.x.1, self.y.1, self.z.1) {
            (Some(norm_x), Some(norm_y), Some(norm_z)) => {
                let (area0, area1, area2) = self.weighted_areas(point);
                let mut norm = norm_x.mult(area2) + norm_y.mult(area1) + norm_z.mult(area0);
                norm.normalize();
                norm
            },
            _ => panic!("Not enough pointers to materials")
        }
    }
}

impl<'a> Shape for Poly<'a> {
    fn get_bbox(&self) -> BoundingBox {
        let min = Vec3::init(
            self[0][0].min(self[1][0].min(self[2][0])),
            self[0][1].min(self[1][1].min(self[2][1])),
            self[0][2].min(self[1][2].min(self[2][2]))
        );

        let max = Vec3::init(
            self[0][0].max(self[1][0].max(self[2][0])),
            self[0][1].max(self[1][1].max(self[2][1])),
            self[0][2].max(self[1][2].max(self[2][2]))
        );

        BoundingBox::init(min, max)
    }

    fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        let p: Vec3 = ray.ori;
        let d: Vec3 = ray.dir;
        let v0: Vec3 = self[0];
        let v1: Vec3 = self[1];
        let v2: Vec3 = self[2];

        let e1: Vec3 = v1 - v0;
        let e2: Vec3 = v2 - v0;

        let h: Vec3 = d.cross(e2);
        let a0: f32 = e1.dot(h);

        if a0 > -0.0000001 && a0 < 0.0000001 {
            return ShapeIntersection::Missed;
        }

        let f: f32 = 1.0 / a0;
        let s: Vec3 = p - v0;
        let u: f32 = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return ShapeIntersection::Missed;
        }

        let q: Vec3 = s.cross(e1);
        let v: f32 = f * d.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return ShapeIntersection::Missed;
        }

        // at this stage we can compute t to find out where
        // the intersection point is on the line
        let t: f32 = f * e2.dot(q);

        match t > 0.0000001 {
            true => ShapeIntersection::Hit(t), // ray intersection
            false => ShapeIntersection::Missed // this means that there is
            // a line intersection but not a ray intersection
        }
    }

    fn get_material(&self) -> Material {
        match self.x.2 {
            Some(material) => material.clone(),
            None => panic!("Poly not associated with a material")
        }
    }

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3 {
        let mut normal = match self.x.1 != None {
            true => self.interpolated_normal(point),
            false => self.static_normal()
        };

        if normal.dot(direction) > 0.0 {
            normal = normal.invert();
        }
        normal
    }

    fn diffuse_color(&self, point: Vec3) -> Color {
        match (self.x.2, self.y.2, self.z.2) {
            (Some(_), Some(_), Some(_)) => self.interpolated_color(point),
            _ => self.get_material().diffuse
        }
    }
}
