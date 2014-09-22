use vec::Vec3;
use ray::Ray;
use scene::material::Material;
use scene::shapes::{Shape, Intersection, Intersected, Missed};
use std::fmt;

#[deriving(Show)]
pub struct Vertex {
    pub mat_index: u32,
    pub has_normal: bool,
    pub position: Vec3,
    pub normal: Vec3
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            mat_index: 0,
            has_normal: false,
            position: Vec3::new(),
            normal: Vec3::new()
        }
    }

    pub fn init(position: Vec3) -> Vertex {
        Vertex {
            mat_index: 0,
            has_normal: false,
            position: position,
            normal: Vec3::new()
        }
    }
}

impl Index<u32, f32> for Vertex {
    fn index<'a>(&'a self, index: &u32) -> &'a f32 {
        match index {
            &0 => &self.position[0],
            &1 => &self.position[1],
            &2 => &self.position[2],
            _ => fail!("Index out of bound: {}", index)
        }
    }
}

#[deriving(Show)]
pub struct Poly {
    pub materials: Vec<Material>,
    pub vertices: [Vertex, ..3],
    pub vertex_material: bool,
    pub vertex_normal: bool
}


impl fmt::Show for [Vertex, ..3] {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self[0]);
        // write!(f, "{}", self[1]);
        // write!(f, "{}", self[2])
        write!(f, "3")
    }
}

impl Poly {
    pub fn new() -> Poly {
        Poly {
            materials: Vec::new(),
            vertices: [
                Vertex::new(),
                Vertex::new(),
                Vertex::new()
            ],
            vertex_material: false,
            vertex_normal: false
        }
    }

    pub fn init() -> Poly {
        let mut poly = Poly::new();
        poly.materials = vec!(Material::new());
        poly
    }
}

impl Shape for Poly {
    fn intersects(&self, ray: Ray) -> Intersection {
        let p: Vec3 = ray.ori;
        let d: Vec3 = ray.dir;
        let v0: Vec3 = self[0].position;
        let v1: Vec3 = self[1].position;
        let v2: Vec3 = self[2].position;

        let e1: Vec3 = v1 - v0;
        let e2: Vec3 = v2 - v0;

        let h: Vec3 = d.cross(e2);
        let a0: f32 = e1.dot(h);

        if a0 > -0.0000001 && a0 < 0.0000001 {
            return Missed;
        }

        let f: f32 = 1.0 / a0;
        let s: Vec3 = p - v0;
        let u: f32 = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return Missed;
        }

        let q: Vec3 = s.cross(e1);
        let v: f32 = f * d.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return Missed;
        }

        // at this stage we can compute t to find out where
        // the intersection point is on the line
        let t: f32 = f * e2.dot(q);

        match t > 0.0000001 {
            true => Intersected(t), // ray intersection
            false => Missed         // this means that there is a line intersection
                                    // but not a ray intersection
        }
    }
}

impl Index<u32, Vertex> for Poly {
    fn index<'a>(&'a self, index: &u32) -> &'a Vertex {
        match index {
            &0 => &self.vertices[0],
            &1 => &self.vertices[1],
            &2 => &self.vertices[2],
            _ => fail!("Index out of bound: {}", index)
        }
    }
}

#[cfg(test)]
mod tests {
    use scene::shapes::poly::{Poly, Vertex};

    #[test]
    fn can_init_vertex() {
        let v = Vertex::new();
        assert_eq!(v.mat_index, 0);
    }

    #[test]
    fn can_init_polygon() {
        let p = Poly::new();
        assert_eq!(p.vertex_material, false);
    }
}