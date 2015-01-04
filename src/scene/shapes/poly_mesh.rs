use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes::{BoundingBox, Shape, ShapeIntersection};
// use scene::shapes::poly::{Vertex, Poly};


// Index into the 'vertices' Vec in PolyMesh
pub type VertexIndex = uint;

// Index into the 'normals' Vec in PolyMesh
pub type NormalIndex = uint;

// Index into the 'materials' Vec in PolyMesh
pub type MaterialIndex = uint;

pub type PolyVertex = (VertexIndex, Option<NormalIndex>, Option<MaterialIndex>)

pub struct Poly {
    pub x: PolyVertex,
    pub y: PolyVertex,
    pub z: PolyVertex
}

pub struct PolyMesh {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Vec3>,
    pub materials: Vec<Material>,
    pub polys: Vec<Poly>
}

impl PolyMesh {
    pub fn new() -> PolyMesh {
        PolyMesh {
            materials: Vec::new(),
            vertices: Vec::new(),
            normals: Vec::new()
        }
    }
}
