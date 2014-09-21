use vec::Vec3;
use scene::material::Material;

pub enum Shape {
    SphereType(Sphere),
    PolySetType(PolySet)
}

pub struct Vertex {
    pub matIndex: u32,
    pub hasNormal: bool,
    pub position: Vec3,
    pub normal: Vec3
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            matIndex: 0,
            hasNormal: false,
            position: Vec3::new(),
            normal: Vec3::new()
        }
    }

    pub fn init(position: Vec3) -> Vertex {
        Vertex {
            matIndex: 0,
            hasNormal: false,
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
    pub vertexMaterial: bool,
    pub vertexNormal: bool
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
            vertexMaterial: false,
            vertexNormal: false
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
}

#[cfg(test)]
mod tests {
    use scene::shapes::{ Poly, Vertex, Sphere };

    #[test]
    fn can_init_vertex() {
        let v = Vertex::new();
        assert_eq!(v.matIndex, 0);
    }

    #[test]
    fn can_init_polygon() {
        let p = Poly::new();
        assert_eq!(p.vertexMaterial, false);
    }

    #[test]
    fn can_init_sphere(){
        let s = Sphere::new();
        assert_eq!(s.radius, 0.0);
    }
}