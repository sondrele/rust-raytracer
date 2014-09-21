use vec::Vec3;
use ray::Ray;
use scene::material::Material;

#[deriving(PartialEq, Show)]
pub enum Intersection {
    Intersected(f32),
    Missed
}

pub enum ShapeType {
    SphereType(Sphere),
    PolySetType(PolySet)
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> Intersection;
}

impl Shape for ShapeType {
    fn intersects(&self, ray: Ray) -> Intersection {
        match self {
            &SphereType(ref sphere) => sphere.intersects(ray),
            &PolySetType(ref polyset) => polyset.intersects(ray)
        }
    }
}

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
}

impl Shape for Poly {
    fn intersects(&self, _: Ray) -> Intersection {
        Intersected(0.0)
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

pub struct PolySet {
    pub materials: Vec<Material>,
    pub polygons: Vec<Poly>
}

impl PolySet {
    pub fn new() -> PolySet {
        PolySet {
            materials: Vec::new(),
            polygons: Vec::new()
        }
    }
}

impl Shape for PolySet {
    fn intersects(&self, _: Ray) -> Intersection {
        Missed
    }
}

pub struct Sphere {
    pub materials: Vec<Material>,
    pub origin: Vec3,
    pub radius: f32,
    pub xaxis: Vec3,
    pub xlength: f32,
    pub yaxis: Vec3,
    pub ylength: f32,
    pub zaxis: Vec3,
    pub zlength: f32
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere{
            materials: Vec::new(),
            origin: Vec3::new(),
            radius: 0.0,
            xaxis: Vec3::new(),
            xlength: 0.0,
            yaxis: Vec3::new(),
            ylength: 0.0,
            zaxis: Vec3::new(),
            zlength: 0.0
        }
    }
    pub fn init(origin: Vec3, radius: f32) -> Sphere {
        Sphere{
            materials: Vec::new(),
            origin: origin,
            radius: radius,
            xaxis: Vec3::new(),
            xlength: 0.0,
            yaxis: Vec3::new(),
            ylength: 0.0,
            zaxis: Vec3::new(),
            zlength: 0.0
        }
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: Ray) -> Intersection {
        // Transforming ray to object space
        let transformed_origin = ray.ori - self.origin;

        //Compute A, B and C coefficients
        let dest = ray.dir;
        let orig = transformed_origin;

        let a: f32 = dest.dot(dest);
        let b: f32 = 2.0 * dest.dot(orig);
        let c: f32 = orig.dot(orig) - (self.radius * self.radius);

        //Find discriminant
        let disc: f32 = b * b - 4.0 * a * c;
        // if discriminant is negative there are no real roots, so return
        // false as ray misses sphere
        if disc < 0.0 {
            return Missed;
        }

        // compute q as described above
        let dist_sqrt = disc.sqrt();
        let q = match b < 0.0 {
            true => (-b - dist_sqrt) / 2.0,
            false => (-b + dist_sqrt) / 2.0
        };

        // compute t0 and t1
        let mut t0 = q / a;
        let mut t1 = c / q;

        // make sure t0 is smaller than t1
        if t0 > t1 {
            // if t0 is bigger than t1 swap them around
            let temp = t0;
            t0 = t1;
            t1 = temp;
        }
        // if t1 is less than zero, the object is in the ray's negative direction
        // and consequently the ray misses the sphere
        if t1 < 0.0 {
            return Missed;
        }

        // if t0 is less than zero, the intersection point is at t1 else the intersection point is at t0
        match t0 < 0.0 {
            true => Intersected(t1),
            false => Intersected(t0)
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::{ Poly, Vertex, Sphere, Shape, Missed, Intersected };

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

    #[test]
    fn can_init_sphere(){
        let s = Sphere::new();
        assert_eq!(s.radius, 0.0);
    }

    #[test]
    fn can_intersect_sphere() {
        let shp = Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        let ray = Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0));
        let res = shp.intersects(ray);

        assert_eq!(res, Intersected(4.0));
    }
}