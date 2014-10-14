use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes;
use scene::shapes::{Shape, ShapeIntersection};
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

impl fmt::Show for [Vertex, ..3] {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {}]", self[0], self[1], self[2])
    }
}

#[deriving(Show)]
pub struct Poly {
    pub materials: Vec<Material>,
    pub vertices: [Vertex, ..3],
    pub vertex_material: bool,
    pub vertex_normal: bool
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

    fn weighted_areas(&self, point: Vec3) -> (f32, f32, f32) {
        let area = Vec3::get_area(self[0].position, self[1].position, self[2].position);
        let area0 = Vec3::get_area(self[0].position, self[1].position, point) / area;
        let area1 = Vec3::get_area(self[2].position, self[0].position, point) / area;
        let area2 = Vec3::get_area(self[1].position, self[2].position, point) / area;

        if area0 > 1.0 || area1 > 1.0 || area2 > 1.0 {
            fail!("Cannot get area, as point is outside of poly")
        }

        (area0, area1, area2)
    }

    fn interpolated_color(&self, point: Vec3) -> Color {
        let (area0, area1, area2) = self.weighted_areas(point);
        self.materials[0].diffuse.mult(area2) + self.materials[1].diffuse.mult(area1) + self.materials[2].diffuse.mult(area0)
    }

    fn static_normal(&self) -> Vec3 {
        let v = self[1].position - self[0].position;
        let w = self[2].position - self[0].position;
        v.cross(w)
    }

    fn interpolated_normal(&self, point: Vec3) -> Vec3 {
        let (area0, area1, area2) = self.weighted_areas(point);
        self[0].normal.mult(area2) + self[1].normal.mult(area1) + self[2].normal.mult(area0)
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

impl Shape for Poly {
    fn intersects(&self, ray: Ray) -> ShapeIntersection {
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
            return shapes::Missed;
        }

        let f: f32 = 1.0 / a0;
        let s: Vec3 = p - v0;
        let u: f32 = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return shapes::Missed;
        }

        let q: Vec3 = s.cross(e1);
        let v: f32 = f * d.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return shapes::Missed;
        }

        // at this stage we can compute t to find out where
        // the intersection point is on the line
        let t: f32 = f * e2.dot(q);

        match t > 0.0000001 {
            true => shapes::Hit(t), // ray intersection
            false => shapes::Missed // this means that there is
            // a line intersection but not a ray intersection
        }
    }

    fn get_material(&self) -> Material {
        self.materials[0]
    }

    fn surface_normal(&self, direction: Vec3, point: Vec3) -> Vec3 {
        let mut normal = match self.vertex_normal {
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
        match self.vertex_material {
            true => self.interpolated_color(point),
            false => self.materials[0].diffuse
        }
    }
}

#[cfg(test)]
mod tests {
    use ray::Ray;
    use vec::Vec3;
    use scene::shapes::{Shape, Hit};
    use scene::shapes::poly::{Poly, Vertex};

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1.0e-6,
                "{} is not approximately equal to {}", a, b);
    }

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
    static SIN_PI_4: f32 = 0.7071067812;

    #[test]
    fn can_intersect_poly() {
        let mut poly = Poly::init();
        poly.vertices[0].position = Vec3::init(2.0, 0.0, -3.0);
        poly.vertices[1].position = Vec3::init(-2.0, 0.0, -3.0);
        poly.vertices[2].position = Vec3::init(0.0, 2.0, -1.0);
        let ray = Ray::init(Vec3::init(0.0, SIN_PI_4, 0.0), Vec3::init(0.0, 0.0, -1.0));

        match poly.intersects(ray) {
            Hit(point) => assert_approx_eq(point, 2.292893),
            _ => fail!("Ray should have intersected at {}", 2.292893 as f32)
        }
    }
}
