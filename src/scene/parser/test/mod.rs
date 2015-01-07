use vec::Vec3;
use scene::parser::SceneParser;
use scene::material::Color;
use scene::Light::{Point, Area, Directional};

static TEST_PATH : &'static str   = "src/scene/parser/test/testdata-";

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

    match parser.parse_light() {
        Point(ref p_light) => {
            assert_eq!(p_light.pos.x, -1.0);
            assert_eq!(p_light.intensity.r_val(), 1.0);
        },
        _ => ()
    }

    match parser.parse_light() {
        Area(a_light) => {
            assert_eq!(a_light.min.x, 0.0);
            assert_eq!(a_light.max.x, 200.0);
            assert_eq!(a_light.intensity.r_val(), 0.0);
        },
        _ => ()
    }

    match parser.parse_light() {
        Directional(ref d_light) => {
            assert_eq!(d_light.dir.x, 0.5);
            assert_eq!(d_light.intensity.r_val(), 0.5);
        },
        _ => ()
    }
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

#[test]
fn can_parse_mesh() {
    let mut parser = scene_parser("per-vertex-polyset");
    let mesh = parser.parse_mesh();
    assert_eq!(mesh.vertices.len(), 9);
    assert_eq!(mesh.normals.len(), 9);
    assert_eq!(mesh.materials.len(), 6);
    assert_eq!(mesh.polys.len(), 3);

    let ref poly0 = mesh[0];
    match poly0.x.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[0]), None => panic!("Should have normal")
    }
    match poly0.x.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[0]), None => panic!("Should have material")
    }
    match poly0.y.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[1]), None => panic!("Should have normal")
    }
    match poly0.y.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[1]), None => panic!("Should have material")
    }
    match poly0.z.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[2]), None => panic!("Should have normal")
    }
    match poly0.z.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[2]), None => panic!("Should have material")
    }

    let ref poly1 = mesh[1];
    match poly1.x.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[3]), None => panic!("Should have material")
    }
    match poly1.y.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[3]), None => panic!("Should have material")
    }
    match poly1.z.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[4]), None => panic!("Should have material")
    }

    let ref poly2 = mesh[2];
    match poly2.x.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[6]), None => panic!("Should have normal")
    }
    match poly2.x.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[5]), None => panic!("Should have material")
    }
    match poly2.y.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[7]), None => panic!("Should have normal")
    }
    match poly2.y.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[4]), None => panic!("Should have material")
    }
    match poly2.z.1 {
        Some(ref n) => assert_eq!(n, &mesh.normals[8]), None => panic!("Should have normal")
    }
    match poly2.z.2 {
        Some(ref m) => assert_eq!(m, &mesh.materials[5]), None => panic!("Should have material")
    }
}
