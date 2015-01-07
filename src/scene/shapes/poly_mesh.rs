use std::rc::Rc;
use std::ops::Deref;
use std::num::FloatMath;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes::{BoundingBox, Shape, ShapeIntersection};

// Index into the 'vertices' Vec in RawMesh
type PointIndex = uint;

// Index into the 'normals' Vec in RawMesh
type NormalIndex = uint;

// Index into the 'materials' Vec in RawMesh
type MaterialIndex = uint;

// Indices for the vertices of a PolyIndex
type VertexIndex = (PointIndex, Option<NormalIndex>, Option<MaterialIndex>);

// The Index-type stored in the RawMesh. These area used to generate a Poly
pub type PolyIndex = (VertexIndex, VertexIndex, VertexIndex);

pub type PolyVertex = (Rc<Vec3>, Option<Rc<Vec3>>, Option<Rc<Material>>);

#[derive(Clone, PartialEq, Show)]
pub struct Poly {
    pub x: PolyVertex,
    pub y: PolyVertex,
    pub z: PolyVertex,
}

#[derive(Clone, PartialEq, Show)]
pub struct Mesh {
    pub vertices: Vec<Rc<Vec3>>,
    pub normals: Vec<Rc<Vec3>>,
    pub materials: Vec<Rc<Material>>,
    pub polys: Vec<Poly>
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            materials: Vec::new(),
            polys: Vec::new()
        }
    }

    pub fn from_data(vertices: Vec<Vec3>, normals: Vec<Vec3>, materials: Vec<Material>,
                     polys: Vec<PolyIndex>) -> Mesh {
        let mut mesh = Mesh::new();
        for &v in vertices.iter() {
            mesh.vertices.push(Rc::new(v));
        }

        for &n in normals.iter() {
            mesh.normals.push(Rc::new(n));
        }

        for &m in materials.iter() {
            mesh.materials.push(Rc::new(m));
        }

        mesh.build_polys(polys);
        mesh
    }

    fn build_polys(&mut self, poly_indices: Vec<PolyIndex>) {
        self.polys = Vec::new();

        for p in poly_indices.iter() {
            let poly = match p {
                &((xv, Some(xn), Some(xm)), (yv, Some(yn), Some(ym)), (zv, Some(zn), Some(zm))) => {
                    Poly {
                        x: (self.vertices[xv].clone(), Some(self.normals[xn].clone()),
                            Some(self.materials[xm].clone())),
                        y: (self.vertices[yv].clone(), Some(self.normals[yn].clone()),
                            Some(self.materials[ym].clone())),
                        z: (self.vertices[zv].clone(), Some(self.normals[zn].clone()),
                            Some(self.materials[zm].clone()))
                    }
                },
                &((xv, Some(xn), None), (yv, Some(yn), None), (zv, Some(zn), None)) => {
                    Poly {
                        x: (self.vertices[xv].clone(), Some(self.normals[xn].clone()), None),
                        y: (self.vertices[yv].clone(), Some(self.normals[yn].clone()), None),
                        z: (self.vertices[zv].clone(), Some(self.normals[zn].clone()), None)
                    }
                },
                &((xv, None, Some(xm)), (yv, None, Some(ym)), (zv, None, Some(zm))) => {
                    Poly {
                        x: (self.vertices[xv].clone(), None, Some(self.materials[xm].clone())),
                        y: (self.vertices[yv].clone(), None, Some(self.materials[ym].clone())),
                        z: (self.vertices[zv].clone(), None, Some(self.materials[zm].clone()))
                    }
                },
                &((xv, None, None), (yv, None, None), (zv, None, None)) => {
                    Poly {
                        x: (self.vertices[xv].clone(), None, None),
                        y: (self.vertices[yv].clone(), None, None),
                        z: (self.vertices[zv].clone(), None, None)
                    }
                },
                _ => panic!("Invalid PolyIndex: {}", p)
            };
            self.polys.push(poly);
        }
    }

    pub fn intersects(&self, ray: &Ray) -> (ShapeIntersection, uint) {
        let mut point = 0.0;
        let mut index = 0;

        let mut has_intersected = false;
        for i in range(0, self.polys.len()) {
            let ref p = self.polys[i];
            match p.intersects(ray) {
                ShapeIntersection::Hit(pt) if !has_intersected => {
                    point = pt;
                    index = i;
                    has_intersected = true;
                }
                ShapeIntersection::Hit(pt) if has_intersected && pt < point => {
                    point = pt;
                    index = i;
                },
                _ => ()
            }
        }
        match has_intersected {
            true => (ShapeIntersection::Hit(point), index),
            false => (ShapeIntersection::Missed, index)
        }
    }
}

impl Poly {

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
        match (&self.x.2, &self.y.2, &self.z.2) {
            (&Some(ref idx_x), &Some(ref idx_y), &Some(ref idx_z)) => {
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
        match (&self.x.1, &self.y.1, &self.z.1) {
            (&Some(ref norm_x), &Some(ref norm_y), &Some(ref norm_z)) => {
                let (area0, area1, area2) = self.weighted_areas(point);
                let mut norm = norm_x.mult(area2) + norm_y.mult(area1) + norm_z.mult(area0);
                norm.normalize();
                norm
            },
            _ => panic!("Not enough pointers to materials")
        }
    }
}

impl Shape for Poly {
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
        match &self.x.2 {
            &Some(ref material) => material.deref().clone(),
            &None => panic!("PolyIndex not associated with a material")
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
        match (&self.x.2, &self.y.2, &self.z.2) {
            (&Some(_), &Some(_), &Some(_)) => self.interpolated_color(point),
            _ => self.get_material().diffuse
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::ShapeIntersection;
    use scene::shapes::poly_mesh::Mesh;

    fn create_mesh<'a>() -> Mesh {
        let mut m = Mesh::new();
        m.vertices = vec!(
            Rc::new(Vec3::init(0.0, 0.0, 0.0)),
            Rc::new(Vec3::init(2.0, 0.0, 0.0)),
            Rc::new(Vec3::init(0.0, 2.0, 0.0)),
            Rc::new(Vec3::init(0.0, 0.0, -2.0)),
            Rc::new(Vec3::init(4.0, 0.0, -2.0)),
            Rc::new(Vec3::init(0.0, 4.0, -2.0)),
            Rc::new(Vec3::init(0.0, 0.0, -1.0)),
            Rc::new(Vec3::init(2.0, 0.0, -1.0)),
            Rc::new(Vec3::init(0.0, 2.0, -1.0)),
            Rc::new(Vec3::init(0.0, 0.0, -3.0)),
            Rc::new(Vec3::init(6.0, 0.0, -3.0)),
            Rc::new(Vec3::init(0.0, 6.0, -3.0))
        );
        m.build_polys(vec!(
            ((0, None, None), (1, None, None), (2, None, None)),
            ((3, None, None), (4, None, None), (5, None, None)),
            ((6, None, None), (7, None, None), (8, None, None)),
            ((9, None, None), (10, None, None), (11, None, None))
        ));
        m
    }

    #[test]
    fn create_poly_from_polymesh() {
        let mesh = create_mesh();
        let ref p = mesh.polys[0];

        assert_eq!(mesh.vertices[0], p.x.0);
        assert_eq!(mesh.vertices[1], p.y.0);
        assert_eq!(mesh.vertices[2], p.z.0);
    }

    #[test]
    fn can_intersect_polymesh() {
        let mesh = create_mesh();

        let ray = Ray::init(Vec3::init(-1.5, 1.0, 1.0), Vec3::init(1.0, 0.0, -1.0));
        match mesh.intersects(&ray) {
            (ShapeIntersection::Hit(pt), i) => {
                assert_eq!(pt, 2.0);
                assert_eq!(i, 2)
            },
            (ShapeIntersection::Missed, _) => panic!("Missed mesh")
        }

    }
}