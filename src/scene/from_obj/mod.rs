extern crate wavefront_obj;

use std::borrow::ToOwned;
use std::io::{BufferedReader, File};

use self::wavefront_obj::{mtl, obj};

fn read_file(path: String) -> String {
    match File::open(&Path::new(path.clone())) {
        Ok(f) => {
            let mut r = BufferedReader::new(f);
            match r.read_to_string() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            }
        },
        Err(e) => panic!("Could not open file with name '{}': {}", path, e)
    }
}

fn parse_mtl(file_name: &str) -> mtl::MtlSet {
    let mtl_string = read_file(file_name.to_owned());
    match mtl::parse(mtl_string) {
        Ok(m) => m,
        Err(e) => panic!("{}", e)
    }
}

fn parse_obj(file_name: &str) -> obj::ObjSet {
    let obj_string = read_file(file_name.to_owned());
    match obj::parse(obj_string) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    }
}


#[cfg(test)]
mod test {
    extern crate test;
    extern crate wavefront_obj;

    use std::borrow::ToOwned;
    use self::wavefront_obj::{obj, mtl};

    use super::{parse_mtl, parse_obj};

    static TEST_PATH : &'static str  = "src/scene/from_obj/test/";


    #[test]
    fn parse_returns_error() {
        let e = obj::parse(String::from_str("no_obj"));

        match e {
            Ok(_) => panic!("There should be no file with name no_obj"),
            Err(_) =>  ()
        }
    }

    #[test]
    fn parse_obj_scene() { // Integration
        let obj_file =
r#"
# Blender v2.71 (sub 0) OBJ File: 'cube.blend'
# www.blender.org
mtllib cube.mtl
o Cube
v 1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 -1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v 1.000000 1.000000 -0.999999
v 0.999999 1.000000 1.000001
v -1.000000 1.000000 1.000000
v -1.000000 1.000000 -1.000000
vt 1.004952 0.498633
vt 0.754996 0.498236
vt 0.755393 0.248279
vt 1.005349 0.248677
vt 0.255083 0.497442
vt 0.255480 0.247485
vt 0.505437 0.247882
vt 0.505039 0.497839
vt 0.754598 0.748193
vt 0.504642 0.747795
vt 0.505834 -0.002074
vt 0.755790 -0.001677
vt 0.005127 0.497044
vt 0.005524 0.247088
usemtl Material
s off
f 1/1 2/2 3/3 4/4
f 5/5 8/6 7/7 6/8
f 1/9 5/10 6/8 2/2
f 2/2 6/8 7/7 3/3
f 3/3 7/7 8/11 4/12
f 5/5 1/13 4/14 8/6
"#;

        let mtl_file =
r#"
# Blender MTL File: 'None'
# Material Count: 2

# name
newmtl Material
# Phong specular coefficient
Ns 96.078431
# ambient color (weighted)
Ka 0.000000 0.000000 0.000000
# diffuse color (weighted)
Kd 0.640000 0.640000 0.640000
# dissolve factor (weighted)
Ks 0.500000 0.500000 0.500000
# optical density (refraction)
Ni 1.000000
# alpha
d 1.000000
# illumination: 0=ambient, 1=ambient+diffuse, 2=ambient+diffuse+specular
illum 2

newmtl None
Ns 0
# ambient
Ka 0.000000 0.000000 0.000000
# diffuse
Kd 0.8 0.8 0.8
# specular
Ks 0.8 0.8 0.8
d 1
illum 2

"#;
        let obj = obj::parse(obj_file.to_owned());
        match obj {
            Ok(obj) => {
                assert_eq!(obj.material_library.as_slice(), "cube.mtl");
                assert_eq!(obj.objects.len(), 1);
            },
            Err(e) => panic!("{}", e)
        }

        let mtl = mtl::parse(mtl_file.to_owned());
        match mtl {
            Ok(mtl) => assert_eq!(mtl.materials.len(), 2),
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn parse_mtl_file() {
        let path = String::from_str(TEST_PATH) + "cube.mtl";
        let mtl = parse_mtl(path.as_slice());
        assert_eq!(mtl.materials.len(), 2)
    }

    #[test]
    fn parse_obj_file() {
        let path = String::from_str(TEST_PATH) + "cube.obj";
        let obj = parse_obj(path.as_slice());
        assert_eq!(obj.material_library.as_slice(), "cube.mtl");
        assert_eq!(obj.objects.len(), 1)
    }

}