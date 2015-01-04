extern crate wavefront_obj;

use std::collections::HashMap;
use std::borrow::ToOwned;
use std::io::{BufferedReader, File};

use self::wavefront_obj::{mtl, obj};
use vec::Vec3;
use scene;
use scene::material::{Color, Material};
use scene::shapes::Primitive;
use scene::shapes::poly;
use scene::parser::SceneParser;

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

fn convert_color(c: mtl::Color) -> Color {
    let mut color = Color::new();
    color.r(c.r as f32);
    color.g(c.g as f32);
    color.b(c.b as f32);
    color
}

fn convert_material(mtl: &mtl::Material) -> Material {
    let mut m = Material::new();
    m.diffuse = convert_color(mtl.color_diffuse);
    m.ambient = convert_color(mtl.color_ambient);
    m.specular = convert_color(mtl.color_specular);
    m.shininess = mtl.specular_coefficient as f32;
    m.transparency = mtl.alpha as f32;
    m
}

fn convert_materials(mtl_set: mtl::MtlSet) -> HashMap<String, Material> {
    let mut m = HashMap::with_capacity(mtl_set.materials.len());
    for mtl in mtl_set.materials.iter() {
        m.insert(mtl.name.clone(), convert_material(mtl));
    }
    m
}

fn parse_obj(file_name: &str) -> obj::ObjSet {
    let obj_string = read_file(file_name.to_owned());
    match obj::parse(obj_string) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    }
}

fn convert_objects(objects: &Vec<obj::Object>,
                   mtls: &HashMap<String, Material>) -> Vec<poly::Poly> {
    let mut polys = Vec::new();
    for obj in objects.iter() {
        let ps = convert_object(obj, mtls);
        polys = polys + ps.as_slice();
    }
    polys
}

fn convert_object(object: &obj::Object, mtls: &HashMap<String, Material>) -> Vec<poly::Poly> {
    let mut polys = Vec::new();
    for geo in object.geometry.iter() {
        let ps = convert_geometry(geo, object, mtls);
        polys = polys + ps.as_slice();
    }
    polys
}

fn convert_geometry(geometry: &obj::Geometry, object: &obj:: Object,
                    mtls: &HashMap<String, Material>) -> Vec<poly::Poly> {
    let m = match &geometry.material_name {
        &Some(ref name) => mtls.get(name).unwrap(),
        &None => panic!("No material name associated with Geometry: {}", geometry)
    };

    let mut polys = Vec::with_capacity(geometry.shapes.len());
    for shp in geometry.shapes.iter() {
        match convert_shape(shp, object) {
            Some(mut poly) => {
                poly.materials = vec!(m.clone());
                polys.push(poly);
            },
            None => ()
        }
    }
    polys
}

fn convert_shape(shp: &obj::Shape, object: &obj::Object) -> Option<poly::Poly> {
    match shp {
        &obj::Shape::Triangle(vertex_x, vertex_y, vertex_z) => {
            let mut p = poly::Poly::new();
            p.vertices[0] = convert_vtindex(vertex_x, object);
            p.vertices[1] = convert_vtindex(vertex_y, object);
            p.vertices[2] = convert_vtindex(vertex_z, object);
            if vertex_x.1 != None {
                p.vertex_normal = true;
            }
            Some(p)
        },
        _ => None
    }
}

fn convert_vtindex(vt: obj::VTIndex, object: &obj::Object) -> poly::Vertex {
    let mut vertex = poly::Vertex::new();
    vertex.position = convert_vertex(object.vertices[vt.0]);
    match vt.2 {
        Some(i) => {
            vertex.normal = convert_vertex(object.normals[i]);
            vertex.has_normal = true;
        },
        None => ()
    }
    vertex
}

fn convert_vertex(v: obj::Vertex) -> Vec3 {
    Vec3::init(v.x as f32, v.y as f32, v.z as f32)
}

pub fn parse_obj_scene<'a>(scene_path: String, obj_path: String) -> scene::BvhScene<'a> {
    let objset = parse_obj(obj_path.as_slice());
    let mtllib = parse_mtl(objset.material_library.as_slice());

    let materials = convert_materials(mtllib);
    let polys = convert_objects(&objset.objects, &materials);
    let prims = polys.map_in_place(|poly| Primitive::Poly(poly));

    let mut parser = SceneParser::new(scene_path);
    let mut scene = parser.parse_scene();
    scene.primitives = scene.primitives + prims.as_slice();
    scene::BvhScene::from_scene(scene)
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
    fn test_parse_returns_error() {
        let e = obj::parse(String::from_str("no_obj"));

        match e {
            Ok(_) => panic!("There should be no file with name no_obj"),
            Err(_) =>  ()
        }
    }

    #[test]
    fn test_parse_obj_scene() {
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

    #[test]
    fn parse_and_convert_mtl() {

    }

}
