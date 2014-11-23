use std::num::Float;
use bmp::Pixel;

#[deriving(PartialEq, Clone, Show)]
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
        c.r(r); c.g(g); c.b(b);
        c
    }

    pub fn r(&mut self, mut r: f32) {
        if r < 0.0 { r = 0.0; }
        if r > 1.0 { r = 1.0; }
        self.r = r;
    }

    pub fn r_val(&self) -> f32 {
        self.r
    }

    pub fn g(&mut self, mut g: f32) {
        if g < 0.0 { g = 0.0; }
        if g > 1.0 { g = 1.0; }
        self.g = g;
    }

    pub fn g_val(&self) -> f32 {
        self.g
    }

    pub fn b(&mut self, mut b: f32) {
        if b < 0.0 { b = 0.0; }
        if b > 1.0 { b = 1.0; }
        self.b = b;
    }

    pub fn b_val(&self) -> f32 {
        self.b
    }

    pub fn scalar(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }

    pub fn as_pixel(&self) -> Pixel {
        Pixel{
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8
        }
    }

    pub fn mult(&self, num: f32) -> Color {
        Color::init(self.r * num, self.g * num, self.b * num)
    }
}

impl Mul<Color, Color> for Color {
    fn mul(&self, col: &Color) -> Color {
        Color::init(self.r * col.r, self.g * col.g, self.b * col.b)
    }
}

impl Add<Color, Color> for Color {
    fn add(&self, col: &Color) -> Color {
        Color::init(self.r + col.r, self.g + col.g, self.b + col.b)
    }
}

#[deriving(Clone, PartialEq, Show)]
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

    pub fn init(diffuse: Color) -> Material {
        let mut material = Material::new();
        material.diffuse = diffuse;
        material
    }

    pub fn is_reflective(&self) -> bool {
        self.specular.scalar() > 0.0
    }

    pub fn is_refractive(&self) -> bool {
        self.transparency > 0.0
    }
}

#[cfg(test)]
mod tests {
    use scene::material::{Color, Material};
    #[test]
    fn color_is_0(){
        let c = Color::new();

        assert!(c.r == 0.0);
        assert!(c.g == 0.0);
        assert!(c.b == 0.0);
    }

    #[test]
    fn color_is_between_0_and_1(){
        let mut c = Color::new();
        c.r(2.0);
        c.g(0.5);
        c.b(-1.0);
        assert!(c.r == 1.0);
        assert!(c.g == 0.5);
        assert!(c.b == 0.0);
    }

    #[test]
    fn material_is_reflective(){
        let mut mat = Material::new();
        mat.specular.r(0.5);
        assert!(mat.is_reflective());
    }

    #[test]
    fn material_is_refractive(){
        let mut mat = Material::new();
        mat.transparency = 0.5;
        assert!(mat.is_refractive());
    }
}
