use std::io;
use std::io::{BufferedReader, File};
use std::from_str::FromStr;

use vec::Vec3;
use scene::{Scene, Camera, Light, PointLight, DirectionalLight, AreaLight};
use scene::material::{Material, Color};
use scene::shapes::{SpherePrim, PolyPrim};
use scene::shapes::sphere::Sphere;
use scene::shapes::poly::{Poly, Vertex};

pub struct SceneParser {
    reader: BufferedReader<File>,
    finished: bool,
    peaked: bool,
    last_token: Option<String>
}

impl SceneParser {
    pub fn new(scene: String) -> SceneParser {
        SceneParser{
            reader: SceneParser::read_file(scene),
            finished: false,
            peaked: false,
            last_token: None
        }
    }

    fn read_file(path: String) -> BufferedReader<File> {
        match File::open(&Path::new(path.clone())) {
            Ok(f) => BufferedReader::new(f),
            Err(e) => fail!("file error: {}, path: {}", e, path.clone())
        }
    }

    fn has_next_token(&self) -> bool {
        !self.finished
    }

    fn peak(&mut self) -> String {
        if self.peaked {
            match self.last_token {
                Some(ref tkn) => { return tkn.clone(); },
                None => fail!("The peaked word does not exist")
            }
        }
        let tkn = self.next_token();
        self.last_token = Some(tkn.clone());
        self.peaked = true;
        return tkn;
    }

    fn next_token(&mut self) -> String {
        if self.peaked {
            let tkn = match self.last_token {
                Some(ref tkn) => tkn.clone(),
                None => fail!("The peaked word does not exist")
            };
            self.last_token = None;
            self.peaked = false;
            return tkn;
        }

        let mut buf = String::new();
        loop {
            let c = match self.reader.read_byte() {
                Ok(c) => c as char,
                Err(e) => match e.kind {
                    io::EndOfFile => {
                        self.finished = true;
                        return buf.to_string();
                    },
                    _ => fail!("Read error: {}", e)
                }
            };
            if !c.is_whitespace() {
                buf.push(c);
            } else if buf.len() > 0 {
                return buf.to_string();
            }
        }
    }

    fn next_num<T:FromStr>(&mut self) -> T {
        let tkn = self.next_token();
        match from_str::<T>(tkn.as_slice()) {
            Some(f) => f,
            None => fail!("Token '{}'", tkn)
        }
    }

    fn consume_next(&mut self) {
        let _ = self.next_token();
    }

    fn check_and_consume(&mut self, token: &str) {
        // TODO: Give a nicer error message than this assert?
        assert_eq!(self.next_token().as_slice(), token)
    }

    fn parse_f32(&mut self, name: &str) -> f32 {
        self.check_and_consume(name);
        self.next_num()
    }

    fn parse_vec3(&mut self, name: &str) -> Vec3 {
        self.check_and_consume(name);
        Vec3::init(self.next_num(), self.next_num(), self.next_num())
    }

    fn parse_color(&mut self, color: &str) -> Color {
        self.check_and_consume(color);
        Color::init(self.next_num(), self.next_num(), self.next_num())
    }

    fn parse_bool(&mut self, name: &str, flag: &str) -> bool {
        self.check_and_consume(name);
        match self.next_token() {
            ref tkn if tkn.as_slice() == flag => true,
            _ => false
        }
    }

    fn parse_light(&mut self) -> Light {
        let keyword = self.next_token();

        let kind = match keyword.as_slice() {
            "point_light" => PointLight,
            "area_light" => AreaLight,
            "directional_light" => DirectionalLight,
            _ => fail!("LightType is not valid: {}", keyword)
        };

        self.check_and_consume("{");

        let light = match kind {
            PointLight => Light {
                kind: kind,
                pos: self.parse_vec3("position"),
                dir: Vec3::new(),
                intensity: self.parse_color("color")
            },
            AreaLight => Light {
                kind: kind,
                pos: self.parse_vec3("position"),
                dir: self.parse_vec3("position"),
                intensity: self.parse_color("color")
            },
            DirectionalLight => Light {
                kind: kind,
                pos: Vec3::new(),
                dir: self.parse_vec3("direction"),
                intensity: self.parse_color("color")
            }
        };

        self.check_and_consume("}");
        light
    }

    fn parse_material(&mut self) -> Material {
        self.check_and_consume("material");
        self.check_and_consume("{");

        let material = Material {
            diffuse: self.parse_color("diffColor"),
            ambient: self.parse_color("ambColor"),
            specular: self.parse_color("specColor"),
            emissive: self.parse_color("emisColor"),
            shininess: self.parse_f32("shininess"),
            transparency: self.parse_f32("ktran")
        };

        self.check_and_consume("}");
        material
    }

    fn parse_sphere(&mut self) -> Sphere {
        self.check_and_consume("sphere");
        self.check_and_consume("{");
        self.check_and_consume("name");
        self.consume_next();
        self.check_and_consume("numMaterials");

        let mut num_materials: i32 = self.next_num();
        let mut sphere = Sphere::new();
        while num_materials > 0 {
            let material = self.parse_material();
            sphere.materials.push(material);
            num_materials -= 1;
        }

        sphere.origin = self.parse_vec3("origin");
        sphere.radius = self.parse_f32("radius");
        sphere.xaxis = self.parse_vec3("xaxis");
        sphere.xlength = self.parse_f32("xlength");
        sphere.yaxis = self.parse_vec3("yaxis");
        sphere.ylength = self.parse_f32("ylength");
        sphere.zaxis = self.parse_vec3("zaxis");
        sphere.zlength = self.parse_f32("zlength");

        self.check_and_consume("}");
        sphere
    }

    fn parse_vertex(&mut self, has_normal: bool, has_material: bool) -> Vertex {
        let mut vertex = Vertex::init(self.parse_vec3("pos"));

        match has_normal {
            true => {
                vertex.normal = self.parse_vec3("norm");
                vertex.has_normal = true;
            },
            false => ()
        }

        match has_material {
            true => {
                self.check_and_consume("materialIndex");
                vertex.mat_index = self.next_num();
            },
            false => ()
        }
        vertex
    }

    fn parse_poly(&mut self, has_normal: bool, has_material: bool) -> Poly {
        self.check_and_consume("poly");
        self.check_and_consume("{");
        self.check_and_consume("numVertices");
        self.consume_next(); // Always 3

        let poly = Poly {
            materials: Vec::new(),
            vertices: [
                self.parse_vertex(has_normal, has_material),
                self.parse_vertex(has_normal, has_material),
                self.parse_vertex(has_normal, has_material)
            ],
            vertex_material: has_material,
            vertex_normal: has_normal
        };
        self.check_and_consume("}");
        poly
    }

    fn parse_polyset(&mut self) -> Vec<Poly> {
        self.check_and_consume("poly_set");
        self.check_and_consume("{");
        self.check_and_consume("name");
        self.consume_next();
        self.check_and_consume("numMaterials");

        let mut num_materials: uint = self.next_num();
        let mut materials = Vec::with_capacity(num_materials);
        while num_materials > 0 {
            let material = self.parse_material();
            materials.push(material);
            num_materials -= 1;
        }

        self.check_and_consume("type");
        self.consume_next(); // TODO: Use this field later
        let per_vertex_normal = self.parse_bool("normType", "PER_VERTEX_NORMAL");
        let material_binding = self.parse_bool("materialBinding", "PER_VERTEX_MATERIAL");
        self.check_and_consume("hasTextureCoords");
        self.consume_next(); // TODO: This field is probably never used
        self.check_and_consume("rowSize");
        self.consume_next(); // TODO: This field is probably never used
        self.check_and_consume("numPolys");

        let mut num_polys: uint = self.next_num();
        let mut polyset = Vec::with_capacity(num_polys);
        while num_polys > 0 {
            let mut poly = self.parse_poly(per_vertex_normal, material_binding);

            match material_binding {
                true => {
                    let (i0, i1, i2) = (poly[0].mat_index, poly[1].mat_index, poly[2].mat_index);
                    poly.materials.push(materials[i0 as uint].clone());
                    poly.vertices[0].mat_index = poly.materials.len() as u32 - 1;

                    if i1 != i0 {
                        poly.materials.push(materials[i1 as uint].clone());
                        poly.vertices[1].mat_index = poly.materials.len() as u32 - 1;
                    } else {
                        poly.vertices[1].mat_index = 0;
                    }

                    if i2 != i1 && i2 != i0 {
                        poly.materials.push(materials[i2 as uint].clone());
                        poly.vertices[2].mat_index = poly.materials.len() as u32 - 1;
                    } else if i2 == i1 && i2 != i0 {
                        poly.vertices[2].mat_index = 1;
                    } else {
                        poly.vertices[2].mat_index = 0;
                    }
                },
                false => {
                    poly.materials.push(materials[0].clone())
                }
            }
            polyset.push(poly);
            num_polys -= 1;
        }

        self.check_and_consume("}");
        polyset
    }

    fn parse_camera(&mut self) -> Camera {
        self.check_and_consume("camera");
        self.check_and_consume("{");
        let camera = Camera {
            pos: self.parse_vec3("position"),
            view_dir: self.parse_vec3("viewDirection"),
            focal_dist: self.parse_f32("focalDistance"),
            ortho_up: self.parse_vec3("orthoUp"),
            vertical_fov: self.parse_f32("verticalFOV")
        };
        self.check_and_consume("}");
        camera
    }

    pub fn parse_scene(&mut self) -> Scene {
        self.check_and_consume("Composer");
        self.check_and_consume("format");
        self.check_and_consume("2.1");
        self.check_and_consume("ascii");

        let mut scene = Scene::new();

        let mut tkn = self.peak();
        while self.has_next_token() {
            match tkn.as_slice() {
                "camera" => scene.camera = self.parse_camera(),
                "sphere" => {
                    let sphere = self.parse_sphere();
                    scene.primitives.push(SpherePrim(sphere));
                },
                "poly_set" => {
                    let mut polyset = self.parse_polyset();

                    for _ in range(0, polyset.len()) {
                        match polyset.pop() {
                            Some(poly) => scene.primitives.push(PolyPrim(poly)),
                            None => fail!("Incorrect amount of polys in polyset")
                        }
                    }
                },
                token if token.ends_with("light") => scene.lights.push(self.parse_light()),
                _ => fail!("Unexpected token: {}", tkn)
            }
            tkn = self.peak();
        }
        scene
    }
}

#[cfg(test)]
mod test_parser {
    use vec::Vec3;
    use parser::SceneParser;
    use scene::material::Color;
    use scene::{ Light, PointLight, DirectionalLight, AreaLight };

    static TEST_PATH : &'static str   = "src/parser/test/testdata-";

    fn scene_parser(name: &str) -> SceneParser {
        let mut test_name = TEST_PATH.to_string();
        test_name.push_str(name);
        test_name.push_str(".txt");
        SceneParser::new(test_name)
    }

    #[test]
    fn can_parse_tokens() {
        let mut parser = scene_parser("light");

        let fst = parser.next_token();
        assert_eq!("point_light", fst.as_slice());

        let snd = parser.next_token();
        assert_eq!("{", snd.as_slice());

        let thrd = parser.next_token();
        assert_eq!("position", thrd.as_slice());

        let frth = parser.next_token();
        assert_eq!("-1", frth.as_slice());

        let mut tkn = "".to_string();
        while parser.has_next_token() {
            tkn = parser.next_token();
        }
        assert_eq!("}", tkn.as_slice());
    }

    #[test]
    fn can_peak_at_next_token() {
        let mut parser = scene_parser("light");

        let tkn = parser.next_token();
        assert_eq!("point_light", tkn.as_slice());

        let tkn = parser.peak();
        assert_eq!("{", tkn.as_slice());

        let tkn = parser.peak();
        assert_eq!("{", tkn.as_slice());

        let tkn = parser.next_token();
        assert_eq!("{", tkn.as_slice());

        let tkn = parser.next_token();
        assert_eq!("position", tkn.as_slice());
    }

    #[test]
    fn can_parse_f32() {
        let mut parser = scene_parser("f32");
        let fst: f32 = parser.next_num();
        assert_eq!(1.5, fst);

        let snd: f32 = parser.next_num();
        assert_eq!(-0.5, snd);
    }

    #[test]
    fn can_parse_position() {
        let mut parser = scene_parser("position");
        let pos: Vec3 = parser.parse_vec3("position");
        assert_eq!(-1.0, pos.x);
        assert_eq!(0.0, pos.y);
        assert_eq!(2.0, pos.z);
    }

    #[test]
    fn can_parse_color() {
        let mut parser = scene_parser("color");
        let color: Color = parser.parse_color("color");
        assert_eq!(1.0, color.r_val());
        assert_eq!(0.0, color.g_val());
        assert_eq!(0.5, color.b_val());
    }

    #[test]
    fn can_parse_light() {
        let mut parser = scene_parser("light");
        let p_light: Light = parser.parse_light();
        assert_eq!(p_light.kind, PointLight);
        assert_eq!(p_light.pos.x, -1.0);
        assert_eq!(p_light.intensity.r_val(), 1.0);

        let a_light = parser.parse_light();
        assert_eq!(a_light.kind, AreaLight);
        assert_eq!(a_light.pos.x, 0.0);
        assert_eq!(a_light.dir.x, 200.0);
        assert_eq!(a_light.intensity.r_val(), 0.0);

        let d_light = parser.parse_light();
        assert_eq!(d_light.kind, DirectionalLight);
        assert_eq!(d_light.dir.x, 0.5);
        assert_eq!(d_light.intensity.r_val(), 0.5);
    }

    #[test]
    fn can_parse_material() {
        let mut parser = scene_parser("material");
        let material = parser.parse_material();
        assert_eq!(material.diffuse.r_val(), 0.56);
        assert_eq!(material.ambient.r_val(), 0.2);
        assert_eq!(material.shininess, 0.2);
        assert_eq!(material.transparency, 0.5);
    }

    #[test]
    fn can_parse_sphere() {
        let mut parser = scene_parser("sphere");
        let sphere = parser.parse_sphere();
        assert_eq!(sphere.materials.len(), 1);
        assert_eq!(sphere.origin.y, -0.5);
        assert_eq!(sphere.radius, 1.5);
    }

    #[test]
    fn can_parse_poly() {
        let mut parser = scene_parser("polygon");
        let poly = parser.parse_poly(false, false);
        assert_eq!(poly[0][0], 0.0);
        assert_eq!(poly[1][0], 0.5);
        assert_eq!(poly[2][0], 10.0);
    }

    #[test]
    fn can_parse_polyset() {
        let mut parser = scene_parser("polyset");
        let polyset = parser.parse_polyset();
        assert_eq!(polyset.len(), 12);

        let ref poly0 = polyset[0];
        assert_eq!(poly0.vertex_material, false);
        assert_eq!(poly0.vertex_normal, false);
        assert_eq!(poly0.materials.len(), 1);
    }

    #[test]
    fn can_parse_per_vertex_polyset() {
        let mut parser = scene_parser("per-vertex-polyset");
        let polyset = parser.parse_polyset();
        assert_eq!(polyset.len(), 3);

        let ref poly0 = polyset[0];
        assert_eq!(poly0.vertex_material, true);
        assert_eq!(poly0.vertex_normal, true);
        assert_eq!(poly0.materials.len(), 3);
        assert_eq!(poly0.materials[0].diffuse, Color::init(0.0, 0.0, 0.0));
        assert_eq!(poly0[0].mat_index, 0);
        assert_eq!(poly0.materials[1].diffuse, Color::init(0.0, 0.0, 1.0));
        assert_eq!(poly0[1].mat_index, 1);
        assert_eq!(poly0.materials[2].diffuse, Color::init(0.0, 1.0, 0.0));
        assert_eq!(poly0[2].mat_index, 2);

        let ref poly1 = polyset[1];
        assert_eq!(poly1.vertex_material, true);
        assert_eq!(poly1.vertex_normal, true);
        assert_eq!(poly1.materials.len(), 2);
        assert_eq!(poly1.materials[0].diffuse, Color::init(0.0, 1.0, 1.0));
        assert_eq!(poly1[0].mat_index, 0);
        assert_eq!(poly1[1].mat_index, 0);
        assert_eq!(poly1.materials[1].diffuse, Color::init(1.0, 0.0, 0.0));
        assert_eq!(poly1[2].mat_index, 1);

        let ref poly2 = polyset[2];
        assert_eq!(poly2.vertex_material, true);
        assert_eq!(poly2.vertex_normal, true);
        assert_eq!(poly2.materials.len(), 2);
        assert_eq!(poly2.materials[0].diffuse, Color::init(1.0, 0.0, 1.0));
        assert_eq!(poly2[0].mat_index, 0);
        assert_eq!(poly2.materials[1].diffuse, Color::init(1.0, 0.0, 0.0));
        assert_eq!(poly2[1].mat_index, 1);
        assert_eq!(poly2[2].mat_index, 0);
    }

    #[test]
    fn can_parse_camera() {
        let mut parser = scene_parser("camera");
        let camera = parser.parse_camera();
        assert_eq!(camera.pos[0], 1.0);
        assert_eq!(camera.view_dir[0], -1.0);
        assert_eq!(camera.focal_dist, 12.0);
        assert_eq!(camera.ortho_up[0], 2.0);
        assert_eq!(camera.vertical_fov, 0.5);
    }

    #[test]
    fn can_parse_scene() {
        let mut parser = scene_parser("scene");
        let scene = parser.parse_scene();
        assert_eq!(scene.lights.len(), 3);
        assert_eq!(scene.primitives.len(), 13);
    }
}
