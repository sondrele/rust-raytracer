use std::io;
use std::io::{ BufferedReader, File };
use vec::Vec3;
use material::Color;
use rscene;
use rscene::{ Light, PointLight, DirectionalLight, AreaLight };

struct SceneParser {
    reader: BufferedReader<File>,
    finished: bool
}

impl SceneParser {
    pub fn new(scene: ~str) -> SceneParser {
        SceneParser{
            reader: SceneParser::read_file(scene),
            finished: false
        }   
    }

    fn Check(keyword: &'static str, to_match: &~str) {
        assert_eq!(keyword, to_match.as_slice())
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

    fn next_token(&mut self) -> ~str {
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

    fn parse_position(&mut self) -> Vec3 {
        let keyword = self.next_token();
        SceneParser::Check("position", &keyword);
        let x: f32 = self.next_f32();
        let y: f32 = self.next_f32();
        let z: f32 = self.next_f32();
        Vec3::init(x, y, z)
    }

    fn parse_direction(&mut self) -> Vec3 {
        let keyword = self.next_token();
        SceneParser::Check("direction", &keyword);
        let x: f32 = self.next_f32();
        let y: f32 = self.next_f32();
        let z: f32 = self.next_f32();
        Vec3::init(x, y, z)
    }


    fn parse_color(&mut self) -> Color {
        let keyword = self.next_token();
        SceneParser::Check("color", &keyword);
        let r: f32 = self.next_f32();
        let g: f32 = self.next_f32();
        let b: f32 = self.next_f32();
        Color::init(r, g, b)
    }

    fn parse_light(&mut self) -> Light {
        let keyword = self.next_token();
        
        let kind: rscene::LightType = if keyword == "point_light".to_owned() {
            PointLight
        } else if keyword == "area_light".to_owned() {
            AreaLight
        } else if keyword == "directional_light".to_owned() {
            DirectionalLight
        } else {
            fail!("LightType is not valid: {}", keyword)
        };
        
        SceneParser::Check("{", &self.next_token());

        let l = match kind {
            PointLight => Light {
                kind: kind,
                pos: self.parse_position(),
                dir: Vec3::new(),
                intensity: self.parse_color()
            },
            AreaLight => Light {
                kind: kind,
                pos: self.parse_position(),
                dir: self.parse_position(),
                intensity: self.parse_color()
            },
            DirectionalLight => Light {
                kind: kind,
                pos: Vec3::new(),
                dir: self.parse_direction(),
                intensity: self.parse_color()
            }
        };
        
        SceneParser::Check("}", &self.next_token());
        
        l
    }
}

#[cfg(test)]
mod tests {
    use parser::SceneParser;
    use vec::Vec3;
    use material::Color;
    use rscene::{ Light, PointLight, DirectionalLight, AreaLight };

    static test_light : &'static str    = "src/parser/test/testdata-light.txt";
    static test_f32 : &'static str      = "src/parser/test/testdata-f32.txt";
    static test_position : &'static str = "src/parser/test/testdata-position.txt";
    static test_colort : &'static str   = "src/parser/test/testdata-color.txt";

    #[test]
    fn can_parse_tokens() {
        let mut parser = SceneParser::new(test_light.to_owned());
        
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
    fn can_parse_f32() {
        let mut parser = SceneParser::new(test_f32.to_owned());
        let fst = parser.next_f32();
        assert_eq!(1.5, fst);

        let snd = parser.next_f32();
        assert_eq!(-0.5, snd);
    }

    #[test]
    fn can_parse_position() {
        let mut parser = SceneParser::new(test_position.to_owned());
        let pos: Vec3 = parser.parse_position();
        assert_eq!(-1.0, pos.x);
        assert_eq!(0.0, pos.y);
        assert_eq!(2.0, pos.z);
    }

    #[test]
    fn can_parse_color() {
        let mut parser = SceneParser::new(test_colort.to_owned());
        let color: Color = parser.parse_color();
        assert_eq!(1.0, color.Rval());
        assert_eq!(0.0, color.Gval());
        assert_eq!(0.5, color.Bval());
    }

    #[test]
    fn can_parse_light() {
        let mut parser = SceneParser::new(test_light.to_owned());
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
}