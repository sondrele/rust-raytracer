use std::num::FloatMath;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes::{BoundingBox, Shape, ShapeIntersection};

// Index into the 'vertices' Vec in PolyMesh
// pub type VertexIndex<'a> = &'a Vec3;
type VertexIndex = uint;

// Index into the 'normals' Vec in PolyMesh
// pub type NormalIndex<'a> = &'a Vec3;
type NormalIndex = uint;

// Index into the 'materials' Vec in PolyMesh
// pub type MaterialIndex<'a> = &'a Material;
type MaterialIndex = uint;

type PolyVertex = (VertexIndex, Option<NormalIndex>, Option<MaterialIndex>);

type PolyIndex = (PolyVertex, PolyVertex, PolyVertex);

pub struct Poly<'a>{
    pub x: (&'a Vec3, Option<&'a Vec3>, Option<&'a Material>),
    pub y: (&'a Vec3, Option<&'a Vec3>, Option<&'a Material>),
    pub z: (&'a Vec3, Option<&'a Vec3>, Option<&'a Material>),
}

pub struct PolyMesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub materials: Vec<Material>,
    poly_indices: Vec<PolyIndex>
}

impl PolyMesh {
    pub fn new() -> PolyMesh {
        PolyMesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            materials: Vec::new(),
            poly_indices: Vec::new()
        }
    }

    pub fn get_poly(&self, index: uint) -> Poly {
        let p = &self.poly_indices[index];
        match p {
            &((xv, Some(xn), Some(xm)), (yv, Some(yn), Some(ym)), (zv, Some(zn), Some(zm))) => {
                Poly {
                    x: (&self.vertices[xv], Some(&self.normals[xn]), Some(&self.materials[xm])),
                    y: (&self.vertices[yv], Some(&self.normals[yn]), Some(&self.materials[ym])),
                    z: (&self.vertices[zv], Some(&self.normals[zn]), Some(&self.materials[zm]))
                }
            },
            &((xv, Some(xn), None), (yv, Some(yn), None), (zv, Some(zn), None)) => {
                Poly {
                    x: (&self.vertices[xv], Some(&self.normals[xn]), None),
                    y: (&self.vertices[yv], Some(&self.normals[yn]), None),
                    z: (&self.vertices[zv], Some(&self.normals[zn]), None)
                }
            },
            &((xv, None, Some(xm)), (yv, None, Some(ym)), (zv, None, Some(zm))) => {
                Poly {
                    x: (&self.vertices[xv], None, Some(&self.materials[xm])),
                    y: (&self.vertices[yv], None, Some(&self.materials[ym])),
                    z: (&self.vertices[zv], None, Some(&self.materials[zm]))
                }
            },
            &((xv, None, None), (yv, None, None), (zv, None, None)) => {
                Poly {
                    x: (&self.vertices[xv], None, None),
                    y: (&self.vertices[yv], None, None),
                    z: (&self.vertices[zv], None, None)
                }
            },
            _ => panic!("Invalid PolyIndex")
        }
    }

    pub fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        let mut point = 0.0;

        let mut has_intersected = false;
        for i in range(0, self.poly_indices.len()) {
            let p = self.get_poly(i);
            match p.intersects(ray) {
                ShapeIntersection::Hit(pt) if !has_intersected => {
                    point = pt;
                    has_intersected = true;
                }
                ShapeIntersection::Hit(pt) if has_intersected && pt < point => {
                    point = pt;
                },
                _ => ()
            }
        }
        match has_intersected {
            true => ShapeIntersection::Hit(point),
            false => ShapeIntersection::Missed
        }
    }
}

impl<'a> Poly<'a> {

    fn weighted_areas(&self, point: Vec3) -> (f32, f32, f32) {
        let area = Vec3::get_area(*self.x.0, *self.y.0, *self.z.0);
        let area0 = Vec3::get_area(*self.x.0, *self.y.0, point) / area;
        let area1 = Vec3::get_area(*self.z.0, *self.x.0, point) / area;
        let area2 = Vec3::get_area(*self.y.0, *self.z.0, point) / area;

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
        let v = *self.y.0 - *self.x.0;
        let w = *self.z.0 - *self.x.0;
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
            self.x.0[0].min(self.y.0[0].min(self.z.0[0])),
            self.x.0[1].min(self.y.0[1].min(self.z.0[1])),
            self.x.0[2].min(self.y.0[2].min(self.z.0[2]))
        );

        let max = Vec3::init(
            self.x.0[0].max(self.y.0[0].max(self.z.0[0])),
            self.x.0[1].max(self.y.0[1].max(self.z.0[1])),
            self.x.0[2].max(self.y.0[2].max(self.z.0[2]))
        );

        BoundingBox::init(min, max)
    }

    fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        let p: Vec3 = ray.ori;
        let d: Vec3 = ray.dir;
        let v0: Vec3 = *self.x.0;
        let v1: Vec3 = *self.y.0;
        let v2: Vec3 = *self.z.0;

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
            None => panic!("PolyIndex not associated with a material")
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

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::ShapeIntersection;
    use scene::shapes::poly_mesh::{Poly, PolyMesh};

    fn create_mesh() -> PolyMesh {
        let mut m = PolyMesh::new();
        m.vertices = vec!(
            Vec3::init(0.0, 0.0, 0.0), Vec3::init(2.0, 0.0, 0.0), Vec3::init(0.0, 2.0, 0.0),
            Vec3::init(0.0, 0.0, -2.0), Vec3::init(4.0, 0.0, -2.0), Vec3::init(0.0, 4.0, -2.0),
            Vec3::init(0.0, 0.0, -1.0), Vec3::init(2.0, 0.0, -1.0), Vec3::init(0.0, 2.0, -1.0),
            Vec3::init(0.0, 0.0, -3.0), Vec3::init(6.0, 0.0, -3.0), Vec3::init(0.0, 6.0, -3.0)
        );
        m.poly_indices = vec!(
            ((0, None, None), (1, None, None), (2, None, None)),
            ((3, None, None), (4, None, None), (5, None, None)),
            ((6, None, None), (7, None, None), (8, None, None)),
            ((9, None, None), (10, None, None), (11, None, None))
        );
        m
    }

    #[test]
    fn create_poly_from_polymesh() {
        let mesh = create_mesh();
        let p: Poly = mesh.get_poly(0);

        assert_eq!(&mesh.vertices[0], p.x.0);
        assert_eq!(&mesh.vertices[1], p.y.0);
        assert_eq!(&mesh.vertices[2], p.z.0);
    }

    #[test]
    fn can_intersect_polymesh() {
        let mesh = create_mesh();

        let ray = Ray::init(Vec3::init(-1.5, 1.0, 1.0), Vec3::init(1.0, 0.0, -1.0));
        match mesh.intersects(&ray) {
            ShapeIntersection::Hit(x) => assert_eq!(x, 2.0),
            ShapeIntersection::Missed => panic!("Missed mesh")
        }

    }
}