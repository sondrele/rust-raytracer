use vec::Vec3;
use ray::Ray;
use scene::material::Material;
use scene::material::Color;
use scene::shapes;
use scene::shapes::{Shape, Intersection};
use scene::shapes::poly::Poly;

#[deriving(Show)]
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

    pub fn init() -> PolySet {
        PolySet {
            materials: vec!(Material::new()),
            polygons: Vec::new()
        }
    }
}

impl Shape for PolySet {
    fn intersects(&self, ray: Ray) -> Intersection {
        let mut intersection = shapes::Missed;

        for p in self.polygons.iter() {
            match p.intersects(ray) {
                shapes::Intersected(point) => {
                    intersection = match intersection {
                        shapes::Intersected(new_point) if new_point < point => {
                            shapes::Intersected(new_point)
                        },
                        _ => shapes::Intersected(point)
                    }
                },
                _ => () // TODO: Match for per_vertex_color
            }
        }
        intersection
    }

    fn get_material(&self) -> Material {
        self.materials[0]
    }

    fn surface_normal(&self, _: Vec3, _: Vec3) -> Vec3 {
        fail!("Can not get surface normal of PolySet")
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::{Shape, Intersected};
    use scene::shapes::polyset::PolySet;
    use scene::shapes::poly::Poly;

    static SIN_PI_4: f32 = 0.7071067812;

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1.0e-6,
                "{} is not approximately equal to {}", a, b);
    }

    #[test]
    fn can_intersect_polyset() {
        let mut poly = Poly::init();
        poly.vertices[0].position = Vec3::init(2.0, 0.0, -3.0);
        poly.vertices[1].position = Vec3::init(-2.0, 0.0, -3.0);
        poly.vertices[2].position = Vec3::init(0.0, 2.0, -1.0);
        let mut set = PolySet::init();
        set.polygons.push(poly);
        let ray = Ray::init(Vec3::init(0.0, SIN_PI_4, 0.0), Vec3::init(0.0, 0.0, -1.0));

        match set.intersects(ray) {
            Intersected(point) => assert_approx_eq(point, 2.292893),
            _ => fail!("Ray should have intersected at {}", 2.292893 as f32)
        }
    }
}