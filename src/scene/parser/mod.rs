use std::io;
use std::io::{BufferedReader, File};
use std::str::FromStr;

use vec::Vec3;
use scene::{BvhScene, Scene, Camera, Light, PointLight, AreaLight, DirectionalLight};
use scene::material::{Material, Color};
use scene::shapes::{sphere, poly};
use scene::shapes::poly_mesh::{Mesh, PolyIndex};
use scene::shapes::Primitive::{Sphere, Poly};

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
            Err(e) => panic!("file error: {}, path: {}", e, path.clone())
        }
    }

    fn has_next_token(&self) -> bool {
        !self.finished
    }

    fn peak(&mut self) -> String {
        if self.peaked {
            match self.last_token {
                Some(ref tkn) => { return tkn.clone(); },
                None => panic!("The peaked word does not exist")
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
                None => panic!("The peaked word does not exist")
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
                    _ => panic!("Read error: {}", e)
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
        match tkn.as_slice().parse() {
            Some(f) => f,
            None => panic!("Token '{}'", tkn)
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

        self.check_and_consume("{");

        let light = match keyword.as_slice() {
            "point_light" => Light::Point(PointLight {
                pos: self.parse_vec3("position"),
                intensity: self.parse_color("color")
            }),
            "area_light" => Light::Area(AreaLight {
                min: self.parse_vec3("position"),
                max: self.parse_vec3("position"),
                intensity: self.parse_color("color")
            }),
            "directional_light" => Light::Directional(DirectionalLight {
                dir: self.parse_vec3("direction"),
                intensity: self.parse_color("color")
            }),
            _ => panic!("LightType is not valid: {}", keyword)
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

    fn parse_sphere(&mut self) -> sphere::Sphere {
        self.check_and_consume("sphere");
        self.check_and_consume("{");
        self.check_and_consume("name");
        self.consume_next();
        self.check_and_consume("numMaterials");

        let mut num_materials: i32 = self.next_num();
        let mut sphere = sphere::Sphere::new();
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

    fn parse_vertex(&mut self, has_normal: bool, has_material: bool) -> poly::Vertex {
        let mut vertex = poly::Vertex::init(self.parse_vec3("pos"));

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

    fn parse_poly(&mut self, has_normal: bool, has_material: bool) -> poly::Poly {
        self.check_and_consume("poly");
        self.check_and_consume("{");
        self.check_and_consume("numVertices");
        self.consume_next(); // Always 3

        let poly = poly::Poly {
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

    fn parse_polyset(&mut self) -> Vec<poly::Poly> {
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

    fn parse_mesh_vertex(&mut self, has_normal: bool,
                         has_material: bool) -> (Vec3, Option<Vec3>, Option<uint>) {
        let pos = self.parse_vec3("pos");
        let norm = match has_normal {
            true => {
                Some(self.parse_vec3("norm"))
            },
            false => None
        };

        let mat_index = match has_material {
            true => {
                self.check_and_consume("materialIndex");
                Some(self.next_num())
            },
            false => None
        };
        (pos, norm, mat_index)
    }


    fn parse_mesh_polys(&mut self, num_polys: uint, per_vertex_normal: bool,
                        material_binding: bool) -> (Vec<PolyIndex>, Vec<Vec3>, Vec<Vec3>) {
        let mut poly_indices = Vec::new();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();

        for _ in range(0, num_polys) {
            self.check_and_consume("poly");
            self.check_and_consume("{");
            self.check_and_consume("numVertices");
            self.check_and_consume("3"); // Always 3

            let (x_pos, x_norm, x_mat_index) = self.parse_mesh_vertex(per_vertex_normal,
                material_binding);
            let (y_pos, y_norm, y_mat_index) = self.parse_mesh_vertex(per_vertex_normal,
                material_binding);
            let (z_pos, z_norm, z_mat_index) = self.parse_mesh_vertex(per_vertex_normal,
                material_binding);

            vertices.push(x_pos);
            vertices.push(z_pos);
            vertices.push(y_pos);

            let l = vertices.len();
            let x_vert_index = l - 3;
            let y_vert_index = l - 2;
            let z_vert_index = l - 1;

            let (x_norm_index, y_norm_index, z_norm_index) = match (x_norm, y_norm, z_norm) {
                (Some(xn), Some(yn), Some(zn)) => {
                    normals.push(xn);
                    normals.push(yn);
                    normals.push(zn);

                    let l = normals.len();
                    (Some(l - 3), Some(l - 2), Some(l - 1))
                },
                _ => (None, None, None)
            };

            self.check_and_consume("}");

            poly_indices.push((
                (x_vert_index, x_norm_index, x_mat_index),
                (y_vert_index, y_norm_index, y_mat_index),
                (z_vert_index, z_norm_index, z_mat_index)
            ))
        }

        (poly_indices, vertices, normals)
    }

    fn parse_mesh(&mut self) -> Mesh {
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

        let num_polys = self.next_num();
        let (poly_indices, vertices, normals) = self.parse_mesh_polys(num_polys,
                                                                      per_vertex_normal,
                                                                      material_binding);
        self.check_and_consume("}");
        Mesh::from_data(vertices, normals, materials, poly_indices)
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

    pub fn parse_scene<'a>(&mut self) -> Scene<'a> {
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
                    scene.primitives.push(Sphere(sphere));
                },
                "poly_set" => {
                    let mut polyset = self.parse_polyset();

                    for _ in range(0, polyset.len()) {
                        match polyset.pop() {
                            Some(poly) => scene.primitives.push(Poly(poly)),
                            None => panic!("Incorrect amount of polys in polyset")
                        }
                    }
                },
                token if token.ends_with("light") => scene.lights.push(self.parse_light()),
                _ => panic!("Unexpected token: {}", tkn)
            }
            tkn = self.peak();
        }
        scene
    }

    pub fn parse_bvh_scene<'a>(&mut self) -> BvhScene<'a> {
        let scene = self.parse_scene();
        BvhScene::from_scene(scene)
    }
}

#[cfg(test)]
mod test;
