use ray::Ray;
use scene::material::Material;
use scene::shapes::{Shape, Intersection, Intersected, Missed};
use scene::shapes::poly::Poly;

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
    fn intersects(&self, ray: Ray) -> Intersection {
        let mut intersection = Missed;

        for p in self.polygons.iter() {
            match p.intersects(ray) {
                Intersected(point) => {
                    intersection = match intersection {
                        Intersected(new_point) if new_point < point => {
                            Intersected(new_point)
                        },
                        _ => Intersected(point)
                    }
                },
                Missed => ()
            }
        }
        intersection
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::{Shape, Missed, Intersected};
    use scene::shapes::polyset::PolySet;
    use scene::shapes::poly::Poly;

    static SIN_PI_4: f32 = 0.7071067812;

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1.0e-6,
                "{} is not approximately equal to {}", a, b);
    }

    #[test]
    fn can_intersect_polyset() {
        let mut poly = Poly::new();
        poly.vertices[0].position = Vec3::init(2.0, 0.0, -3.0);
        poly.vertices[1].position = Vec3::init(-2.0, 0.0, -3.0);
        poly.vertices[2].position = Vec3::init(0.0, 2.0, -1.0);
        let mut set = PolySet::new();
        set.polygons.push(poly);
        let ray = Ray::init(Vec3::init(0.0, SIN_PI_4, 0.0), Vec3::init(0.0, 0.0, -1.0));

        match set.intersects(ray) {
            Intersected(point) => assert_approx_eq(point, 2.292893),
            Missed => fail!("Ray should have intersected at {}", 2.292893 as f32)
        }
    }
}