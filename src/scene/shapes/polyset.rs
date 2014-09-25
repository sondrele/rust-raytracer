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

    fn get_color(&self) -> Color {
        // Color::init(1.0, 1.0, 1.0)
        Color::init(0.0, 0.0, 0.0)
    }
}

impl Shape for PolySet {
    fn intersects(&self, ray: Ray) -> Intersection {
        let color = self.get_color();
        let mut intersection = shapes::Missed;

        for p in self.polygons.iter() {
            match p.intersects(ray) {
                shapes::Intersected(point) => {
                    intersection = match intersection {
                        shapes::Intersected(new_point) if new_point < point => {
                            shapes::IntersectedWithColor(new_point, color)
                        },
                        _ => shapes::IntersectedWithColor(point, color)
                    }
                },
                shapes::IntersectedWithIndex(point, index) => {
                    intersection = match intersection {
                        shapes::Intersected(new_point) if new_point < point => {
                            shapes::IntersectedWithColor(new_point, self.materials[index].diffuse)
                        },
                        _ => shapes::IntersectedWithColor(point, self.materials[index].diffuse)
                    }
                },
                _ => () // TODO: Match for per_vertex_color
            }
        }
        intersection
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::{Shape, IntersectedWithColor};
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
            IntersectedWithColor(point, _) => assert_approx_eq(point, 2.292893),
            _ => fail!("Ray should have intersected at {}", 2.292893 as f32)
        }
    }
}