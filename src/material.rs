
pub struct Color {
    r: f32,
    g: f32,
    b: f32
}

impl Color {
    pub fn new() -> Color {
        Color{ r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn init(r: f32, g: f32, b: f32) -> Color {
        let mut c = Color::new();
        c.R(r); c.G(g); c.B(b);
        c
    }

    pub fn R(&mut self, mut r: f32) {
        if r < 0.0 { r = 0.0; } 
        if r > 1.0 { r = 1.0; }
        self.r = r;
    }

    pub fn Rval(&self) -> f32 {
        self.r
    }

    pub fn G(&mut self, mut g: f32) {
        if g < 0.0 { g = 0.0; } 
        if g > 1.0 { g = 1.0; }
        self.g = g;
    }

    pub fn Gval(&self) -> f32 {
        self.g
    }

    pub fn B(&mut self, mut b: f32) {
        if b < 0.0 { b = 0.0; } 
        if b > 1.0 { b = 1.0; }
        self.b = b;
    }

    pub fn Bval(&self) -> f32{
        self.b
    }
    
    pub fn scalar(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }
}

pub struct Material {
    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
    pub emissive: Color,
    pub shininess: f32,
    pub transparency: f32
}

impl Material {
    pub fn new() -> Material {
        Material{
            diffuse: Color::new(),
            ambient: Color::new(),
            specular: Color::new(),
            emissive: Color::new(),
            shininess: 0.0,
            transparency: 0.0
        }
    }

    fn is_reflective(&self) -> bool {
        self.specular.scalar() > 0.0
    }

    fn is_refractive(&self) -> bool {
        self.transparency > 0.0
    }
}


#[test]
fn Color_is_0(){
    let c = Color::new();

    assert!(c.r == 0.0);
    assert!(c.g == 0.0);
    assert!(c.b == 0.0);
}

#[test]
fn Color_is_between_0_and_1(){
    let mut c = Color::new();
    c.R(2.0);
    c.G(0.5);
    c.B(-1.0);
    assert!(c.r == 1.0);
    assert!(c.g == 0.5);
    assert!(c.b == 0.0);
}

#[test]
fn Material_is_reflective(){
    let mut mat = Material::new();
    mat.specular.R(0.5);
    assert!(mat.is_reflective());
}

#[test]
fn Material_is_refractive(){
    let mut mat = Material::new();
    mat.transparency = 0.5;
    assert!(mat.is_refractive());
}