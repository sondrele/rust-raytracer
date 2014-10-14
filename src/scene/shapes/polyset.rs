use scene::material::Material;
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
        let mut polyset = PolySet::new();
        polyset.materials = vec!(Material::new());
        polyset
    }
}
