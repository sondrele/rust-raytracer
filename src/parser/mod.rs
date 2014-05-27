use std::io;
use std::io::{ BufferedReader, File };
use vec::Vec3;
use scene::{ Scene, Camera, Light, PointLight, DirectionalLight, AreaLight };
use scene::material::{ Material, Color };
use scene::shapes::{ PolySetType, SphereType, Sphere, PolySet, Poly, Vertex };

struct SceneParser {
    reader: BufferedReader<File>,
    finished: bool,
    peaked: bool,
    lastToken: Option<~str>
}

impl SceneParser {
    pub fn new(scene: ~str) -> SceneParser {
        SceneParser{
            reader: SceneParser::read_file(scene),
            finished: false,
            peaked: false,
            lastToken: None
        }   
    }

    fn read_file(path: ~str) -> BufferedReader<File> {
        match File::open(&Path::new(path.clone())) {
            Ok(f) => BufferedReader::new(f),
            Err(e) => fail!("file error: {}, path: {}", e, path.clone())
        }
    }

    fn has_next_token(&self) -> bool {
        !self.finished
    }

    fn peak(&mut self) -> ~str {
        if self.peaked {
            match self.lastToken {
                Some(ref tkn) => { return tkn.clone(); },
                None => fail!("The peaked word does not exist")
            }
        }
        let tkn = self.next_token();
        self.lastToken = Some(tkn.clone());
        self.peaked = true;
        return tkn;
    }

    fn next_token(&mut self) -> ~str {
        if self.peaked {
            // let tkn = self.lastToken.unwrap(); <- Hva er denne feilmeldingen?
            let tkn = match self.lastToken {
                Some(ref tkn) => tkn.clone(),
                None => fail!("The peaked word does not exist")
            };
            self.lastToken = None;
            self.peaked = false;
            return tkn;
        }
        let mut buf = StrBuf::new();
        loop {
            let c = match self.reader.read_byte() {
                Ok(c) => c as char,
                Err(e) => match e.kind {
                    io::EndOfFile => {
                        self.finished = true;
                        return buf.as_slice().to_owned();
                    },
                    _ => fail!("Read error: {}", e)
                }
            };
            if !c.is_whitespace() {
                buf.push_char(c);
            } else if buf.len() > 0 {
                return buf.as_slice().to_owned();
            }
        }
    }

    fn next_f32(&mut self) -> f32 {
        let tkn = self.next_token();
        match from_str::<f32>(tkn) {
            Some(f) => f,
            None => fail!("Token '{}' can not be parsed as f32", tkn)
        }
    }

    fn next_i32(&mut self) -> i32 {
        let tkn = self.next_token();
        match from_str::<i32>(tkn) {
            Some(f) => f,
            None => fail!("Token '{}' can not be parsed as i32", tkn)
        }
    }

    fn consume_next(&mut self) {
        let _ = self.next_token();
    }

    fn check_and_consume(&mut self, token: &'static str) {
        // TODO: Give a nicer error message than this assert?
        assert_eq!(self.next_token(), token.to_owned())
    }

    fn parse_f32(&mut self, name: &'static str) -> f32 {
        self.check_and_consume(name);
        self.next_f32()
    }

    fn parse_vec3(&mut self, name: &'static str) -> Vec3 {
        self.check_and_consume(name);
        Vec3::init(self.next_f32(), self.next_f32(), self.next_f32())
    }

    fn parse_color(&mut self, color: &'static str) -> Color {
        self.check_and_consume(color);
        Color::init(self.next_f32(), self.next_f32(), self.next_f32())
    }

    fn parse_light(&mut self) -> Light {
        let keyword = self.next_token();
        
        let kind = if keyword == "point_light".to_owned() {
            PointLight
        } else if keyword == "area_light".to_owned() {
            AreaLight
        } else if keyword == "directional_light".to_owned() {
            DirectionalLight
        } else {
            fail!("LightType is not valid: {}", keyword)
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

        let mut numMaterials = self.next_i32();
        let mut sphere = Sphere::new();
        while numMaterials > 0 {
            let material = self.parse_material();
            sphere.materials.push(material);
            numMaterials -= 1;
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

    fn parse_polygon(&mut self) -> Poly {
        self.check_and_consume("poly");
        self.check_and_consume("{");
        self.check_and_consume("numVertices");
        self.consume_next();

        let poly = Poly {
            materials: Vec::new(),
            vertices: [
                Vertex::init(self.parse_vec3("pos")),
                Vertex::init(self.parse_vec3("pos")),
                Vertex::init(self.parse_vec3("pos"))
            ],
            vertexMaterial: false,
            vertexNormal: false
        };
        self.check_and_consume("}");
        poly 
    }

    fn parse_polygon_set(&mut self) -> PolySet {
        self.check_and_consume("poly_set");
        self.check_and_consume("{");
        self.check_and_consume("name");
        self.consume_next();
        self.check_and_consume("numMaterials");

        let mut polyset = PolySet::new();
        let mut numMaterials = self.next_i32();
        while numMaterials > 0 {
            let material = self.parse_material();
            polyset.materials.push(material);
            numMaterials -= 1;
        }

        self.check_and_consume("type");
        self.consume_next(); // TODO: Use this field later
        self.check_and_consume("normType");
        self.consume_next(); // TODO: Use this field later
        self.check_and_consume("materialBinding");
        self.consume_next(); // TODO: Use this field later
        self.check_and_consume("hasTextureCoords");
        self.consume_next(); // TODO: Use this field later
        self.check_and_consume("rowSize");
        self.consume_next(); // TODO: This field is probably never used
        self.check_and_consume("numPolys");

        let mut numPolys = self.next_i32();
        while numPolys > 0 {
            let poly = self.parse_polygon();
            polyset.polygons.push(poly);
            numPolys -= 1;
        }

        self.check_and_consume("}");
        polyset
    }

    fn parse_camera(&mut self) -> Camera {
        self.check_and_consume("camera");
        self.check_and_consume("{");
        let camera = Camera {
            pos: self.parse_vec3("position"),
            viewDir: self.parse_vec3("viewDirection"),
            focalDist: self.parse_f32("focalDistance"),
            orthoUp: self.parse_vec3("orthoUp"),
            verticalFOV: self.parse_f32("verticalFOV")
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
            if tkn == "camera".to_owned() {
                scene.camera = self.parse_camera();
            } else if tkn.ends_with("light") {
                scene.lights.push(self.parse_light());
            } else if tkn == "sphere".to_owned() {
                let sphere = self.parse_sphere();
                scene.shapes.push(SphereType(sphere));
            } else if tkn == "poly_set".to_owned() {
                let polyset = self.parse_polygon_set();
                scene.shapes.push(PolySetType(polyset));
            } else {
                fail!("Unexpected token: {}", tkn);
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

    static path : &'static str   = "src/parser/test/testdata-";

    fn scene_parser(name: &'static str) -> SceneParser {
        let name = path.to_owned()
            .append(name)
            .append(".txt");
        SceneParser::new(name)
    }

    #[test]
    fn can_parse_tokens() {
        let mut parser = scene_parser("light");
        
        let fst = parser.next_token();
        assert_eq!("point_light".to_owned(), fst);

        let snd = parser.next_token();
        assert_eq!("{".to_owned(), snd);

        let thrd = parser.next_token();
        assert_eq!("position".to_owned(), thrd);
        
        let frth = parser.next_token();
        assert_eq!("-1".to_owned(), frth);

        let mut tkn = "".to_owned();
        while parser.has_next_token() {
            tkn = parser.next_token();
        }
        assert_eq!("}".to_owned(), tkn);
    }

    #[test]
    fn can_peak_at_next_token() {
        let mut parser = scene_parser("light");
        
        let tkn = parser.next_token();
        assert_eq!("point_light".to_owned(), tkn);

        let tkn = parser.peak();
        assert_eq!("{".to_owned(), tkn);

        let tkn = parser.peak();
        assert_eq!("{".to_owned(), tkn);

        let tkn = parser.next_token();
        assert_eq!("{".to_owned(), tkn);
        
        let tkn = parser.next_token();
        assert_eq!("position".to_owned(), tkn);
    }

    #[test]
    fn can_parse_f32() {
        let mut parser = scene_parser("f32");
        let fst = parser.next_f32();
        assert_eq!(1.5, fst);

        let snd = parser.next_f32();
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
        assert_eq!(1.0, color.Rval());
        assert_eq!(0.0, color.Gval());
        assert_eq!(0.5, color.Bval());
    }

    #[test]
    fn can_parse_light() {
        let mut parser = scene_parser("light");
        let p_light: Light = parser.parse_light();
        assert_eq!(p_light.kind, PointLight);
        assert_eq!(p_light.pos.x, -1.0);
        assert_eq!(p_light.intensity.Rval(), 1.0);

        let a_light = parser.parse_light();
        assert_eq!(a_light.kind, AreaLight);
        assert_eq!(a_light.pos.x, 0.0);
        assert_eq!(a_light.dir.x, 200.0);
        assert_eq!(a_light.intensity.Rval(), 0.0);

        let d_light = parser.parse_light();
        assert_eq!(d_light.kind, DirectionalLight);
        assert_eq!(d_light.dir.x, 0.5);
        assert_eq!(d_light.intensity.Rval(), 0.5);   
    }

    #[test]
    fn can_parse_material() {
        let mut parser = scene_parser("material");
        let material = parser.parse_material();
        assert_eq!(material.diffuse.Rval(), 0.56);
        assert_eq!(material.ambient.Rval(), 0.2);
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
    fn can_parse_polygon() {
        let mut parser = scene_parser("polygon");
        let poly = parser.parse_polygon();
        assert_eq!(poly[0][0], 0.0);
        assert_eq!(poly[1][0], 0.5);
        assert_eq!(poly[2][0], 10.0);
    }

    #[test]
    fn can_parse_polygonset() {
        let mut parser = scene_parser("polyset");
        let polyset = parser.parse_polygon_set();
        assert_eq!(polyset.materials.len(), 1);
        assert_eq!(polyset.polygons.len(), 12);
    }

    #[test]
    fn can_parse_camera() {
        let mut parser = scene_parser("camera");
        let camera = parser.parse_camera();
        assert_eq!(camera.pos[0], 1.0);
        assert_eq!(camera.viewDir[0], -1.0);
        assert_eq!(camera.focalDist, 12.0);
        assert_eq!(camera.orthoUp[0], 2.0);
        assert_eq!(camera.verticalFOV, 0.5);
    }

    #[test]
    fn can_parse_scene() {
        let mut parser = scene_parser("scene");
        let scene = parser.parse_scene();
        assert_eq!(scene.lights.len(), 3);
        assert_eq!(scene.shapes.len(), 2);
    }
}