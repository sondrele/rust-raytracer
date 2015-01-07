use std::rc::Rc;
use std::ops::Deref;
use std::num::FloatMath;
use std::ops::Index;

use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes::{BoundingBox, Primitive, Shape, ShapeIntersection};
use scene::shapes::Primitive::MeshPoly;

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

#[derive(Clone, PartialEq, Show)]
pub struct PolyVertex {
    pub position: Rc<Vec3>,
    pub normal: Option<Rc<Vec3>>,
    pub material: Rc<Material>
}

impl PolyVertex {
    fn init(position: Rc<Vec3>, normal: Option<Rc<Vec3>>, material: Rc<Material>) -> PolyVertex {
        PolyVertex {
            position: position,
            normal: normal,
            material: material
        }
    }
}

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
    pub polys: Vec<Primitive>
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
                    MeshPoly(Poly {
                        x: PolyVertex::init(
                            self.vertices[xv].clone(),
                            Some(self.normals[xn].clone()),
                            self.materials[xm].clone()
                        ),
                        y: PolyVertex::init(
                            self.vertices[yv].clone(),
                            Some(self.normals[yn].clone()),
                            self.materials[ym].clone()
                        ),
                        z: PolyVertex::init(
                            self.vertices[zv].clone(),
                            Some(self.normals[zn].clone()),
                            self.materials[zm].clone()
                        )
                    })
                },
                &((xv, Some(xn), None), (yv, Some(yn), None), (zv, Some(zn), None)) => {
                    MeshPoly(Poly {
                        x: PolyVertex::init(
                            self.vertices[xv].clone(),
                            Some(self.normals[xn].clone()),
                            self.materials[0].clone()),
                        y: PolyVertex::init(
                            self.vertices[yv].clone(),
                            Some(self.normals[yn].clone()),
                            self.materials[0].clone()
                        ),
                        z: PolyVertex::init(
                            self.vertices[zv].clone(),
                            Some(self.normals[zn].clone()),
                            self.materials[0].clone()
                        )
                    })
                },
                &((xv, None, Some(xm)), (yv, None, Some(ym)), (zv, None, Some(zm))) => {
                    MeshPoly(Poly {
                        x: PolyVertex::init(
                            self.vertices[xv].clone(),
                            None,
                            self.materials[xm].clone()
                        ),
                        y: PolyVertex::init(
                            self.vertices[yv].clone(),
                            None,
                            self.materials[ym].clone()
                        ),
                        z: PolyVertex::init(
                            self.vertices[zv].clone(),
                            None,
                            self.materials[zm].clone()
                        )
                    })
                },
                &((xv, None, None), (yv, None, None), (zv, None, None)) => {
                    MeshPoly(Poly {
                        x: PolyVertex::init(
                            self.vertices[xv].clone(),
                            None,
                            self.materials[0].clone()
                        ),
                        y: PolyVertex::init(
                            self.vertices[yv].clone(),
                            None,
                            self.materials[0].clone()
                        ),
                        z: PolyVertex::init(
                            self.vertices[zv].clone(),
                            None,
                            self.materials[0].clone()
                        )
                    })
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

impl Index<uint> for Mesh {
    type Output = Poly;

    fn index<'a>(&'a self, index: &uint) -> &'a Poly {
        match self.polys[*index] {
            MeshPoly(ref p) => p,
            _ => panic!("Mesh should not contain other primitives than MeshPoly")
        }
    }
}

impl Poly {
    fn weighted_areas(&self, point: Vec3) -> (f32, f32, f32) {
        let area = Vec3::get_area(*self.x.position, *self.y.position, *self.z.position);
        let area0 = Vec3::get_area(*self.x.position, *self.y.position, point) / area;
        let area1 = Vec3::get_area(*self.z.position, *self.x.position, point) / area;
        let area2 = Vec3::get_area(*self.y.position, *self.z.position, point) / area;

        if area0 > 1.0 || area1 > 1.0 || area2 > 1.0 {
            panic!("Cannot get area, as point is outside of poly")
        }

        (area0, area1, area2)
    }

    fn interpolated_color(&self, point: Vec3) -> Color {
        let (area0, area1, area2) = self.weighted_areas(point);
        self.x.material.diffuse.mult(area2) + self.y.material.diffuse.mult(area1)
            + self.z.material.diffuse.mult(area0)
    }

    fn static_normal(&self) -> Vec3 {
        let v = *self.y.position - *self.x.position;
        let w = *self.z.position - *self.x.position;
        v.cross(w)
    }

    fn interpolated_normal(&self, point: Vec3) -> Vec3 {
        match (&self.x.normal, &self.y.normal, &self.z.normal) {
            (&Some(ref norm_x), &Some(ref norm_y), &Some(ref norm_z)) => {
                let (area0, area1, area2) = self.weighted_areas(point);
                norm_x.mult(area2) + norm_y.mult(area1) + norm_z.mult(area0)
            },
            _ => panic!("Not enough pointers to materials")
        }
    }
}

impl Shape for Poly {
    fn get_bbox(&self) -> BoundingBox {
        let min = Vec3::init(
            self.x.position[0].min(self.y.position[0].min(self.z.position[0])),
            self.x.position[1].min(self.y.position[1].min(self.z.position[1])),
            self.x.position[2].min(self.y.position[2].min(self.z.position[2]))
        );

        let max = Vec3::init(
            self.x.position[0].max(self.y.position[0].max(self.z.position[0])),
            self.x.position[1].max(self.y.position[1].max(self.z.position[1])),
            self.x.position[2].max(self.y.position[2].max(self.z.position[2]))
        );

        BoundingBox::init(min, max)
    }

    fn intersects(&self, ray: &Ray) -> ShapeIntersection {
        let p: Vec3 = ray.ori;
        let d: Vec3 = ray.dir;
        let v0: Vec3 = *self.x.position;
        let v1: Vec3 = *self.y.position;
        let v2: Vec3 = *self.z.position;

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
        self.x.material.deref().clone()
    }

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3 {
        let mut normal = match self.x.normal.is_some() {
            true => self.interpolated_normal(point),
            false => self.static_normal()
        };
        normal.normalize();

        if normal.dot(direction) > 0.0 {
            normal = normal.invert();
        }
        normal
    }

    fn diffuse_color(&self, point: Vec3) -> Color {
        self.interpolated_color(point)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use vec::Vec3;
    use ray::Ray;
    use scene::material::Material;
    use scene::shapes::ShapeIntersection;
    use scene::shapes::poly_mesh::Mesh;

    fn create_mesh<'a>() -> Mesh {
        let mut m = Mesh::new();
        m.materials.push(Rc::new(Material::new()));
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
        let ref p = mesh[0];

        assert_eq!(mesh.vertices[0], p.x.position);
        assert_eq!(mesh.vertices[1], p.y.position);
        assert_eq!(mesh.vertices[2], p.z.position);
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