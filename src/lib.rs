extern crate bmp;

use std::rand::{random, Open01};
use std::num::Float;
use std::num::FloatMath;

use bmp::Image;

use vec::Vec3;
use ray::Ray;
use scene::{IntersectableScene, Light};
use scene::LightType::{DirectionalLight, PointLight, AreaLight};
use scene::SceneIntersection::{Intersected, Missed};
use scene::material::Color;
use scene::intersection::Intersection;

pub mod vec;
pub mod ray;
pub mod scene;

static SCALE: f32 = 1000.0;
static AREA_SAMPLES: uint = 10;

pub struct RayTracer<'a> {
    width: uint,
    height: uint,
    num_samples: uint,
    depth: uint,
    center: Vec3,
    camera_pos: Vec3,
    parallel_up: Vec3,
    parallel_right: Vec3,
    vertical_fov: f32,
    horizontal_fov: f32,
    scene: Option<Box<IntersectableScene<'a> + 'a>>
}

impl<'a> RayTracer<'a> {
    pub fn new() -> RayTracer<'a> {
        RayTracer {
            width: 0,
            height: 0,
            num_samples: 1,
            depth: 0,
            center: Vec3::new(),
            camera_pos: Vec3::new(),
            parallel_up: Vec3::new(),
            parallel_right: Vec3::new(),
            vertical_fov: 0.0,
            horizontal_fov: 0.0,
            scene: None
        }
    }

    pub fn init(width: uint, height: uint, depth: uint, num_samples: uint) -> RayTracer<'a> {
        let mut raytracer = RayTracer::new();
        raytracer.width = width;
        raytracer.height = height;
        raytracer.depth = depth;
        raytracer.num_samples = num_samples;
        raytracer
    }

    pub fn set_scene(&mut self, scene: Box<IntersectableScene<'a> + 'a>) {
        self.scene = Some(scene);
        self.setup_camera();
    }

    fn setup_camera(&mut self) {
        let cam = match self.scene {
            Some(ref scene) => scene.get_camera(),
            None => panic!("RayTracer has not been assigned any Scene")
        };

        self.parallel_right = cam.view_dir.cross(cam.ortho_up);
        self.parallel_up = self.parallel_right.cross(cam.view_dir);
        self.parallel_right.normalize();
        self.parallel_up.normalize();

        self.vertical_fov = cam.vertical_fov;
        self.horizontal_fov = cam.vertical_fov * (self.width as f32 / self.height as f32);
        self.camera_pos = cam.pos;
        self.center = cam.pos + cam.view_dir.mult(SCALE);
    }

    fn vertical_plane(&self) -> Vec3 {
        let f = (self.vertical_fov / 2.0).tan() * SCALE;
        self.parallel_up.mult(f)
    }

    fn horizontal_plane(&self) -> Vec3 {
        let f = (self.horizontal_fov / 2.0).tan() * SCALE;
        self.parallel_right.mult(f)
    }

    fn compute_ray(&self, x: f32, y: f32) -> Ray {
        let (x, y) = (x * (1.0 / self.width as f32), y * (1.0 / self.height as f32));
        let dx = self.horizontal_plane().mult(2.0 * x - 1.0);
        let dy = self.vertical_plane().mult(2.0 * y - 1.0);
        let mut dir = self.center + dx + dy;
        dir.normalize();
        Ray::init(self.camera_pos, dir)
    }

    fn shadow_scalar<'b>(scene: &'a Box<IntersectableScene<'a> + 'a>, light: &Light, intersection: &Intersection, num_samples: uint, depth: uint) -> Color {
        if depth <= 0 {
            return Color::new();
        }

        let dest: Vec3 = light.pos;
        let (ori, dirs) = match light.kind {
            DirectionalLight => {
                let dir = light.dir.invert();
                let ori = intersection.point() + dir.mult(0.0001);
                (ori, vec!(dir))
            },
            PointLight => {
                let mut ori = intersection.point();
                let mut dir = dest - ori;
                dir.normalize();
                ori = ori + dir.mult(0.0001);
                (ori, vec!(dir))
            },
            AreaLight => {
                let ori = intersection.point() + intersection.surface_normal().mult(0.0001);
                let dirs = Vec::from_fn(num_samples, |_| -> Vec3 {
                    let Open01(rx) = random::<Open01<f32>>();
                    let Open01(ry) = random::<Open01<f32>>();
                    let Open01(rz) = random::<Open01<f32>>();
                    let mut dx = (light.pos[0] - light.dir[0]).abs() * 0.5;
                    let mut dy = (light.pos[1] - light.dir[1]).abs() * 0.5;
                    let mut dz = (light.pos[2] - light.dir[2]).abs() * 0.5;
                    dx = dx - rx * (dx * 2.0);
                    dy = dy - ry * (dy * 2.0);
                    dz = dz - rz * (dz * 2.0);
                    let mut dir = Vec3::init(light.pos[0] + dx, light.pos[1] + dy, light.pos[2] + dz) - ori;
                    dir.normalize();
                    dir
                });
                (ori, dirs)
            }
        };

        let mut shade: f32 = 0.0;

        for &dir in dirs.iter() {
            let shadow = Ray::init(ori, dir);
            shade += match scene.intersects(&shadow) {
                Intersected(intersection) => {
                    let material = intersection.material();
                    if material.transparency == 0.0 {
                        match light.kind {
                            PointLight if ori.distance(intersection.point()) > ori.distance(dest) =>
                                1.0, // Intersects with object behind light source
                            _ => 0.0 // Hit something before directional light, ignoring area light
                        }
                    } else { // Shape is transparent, continue recursively
                        material.transparency * RayTracer::shadow_scalar(scene, light, &intersection, num_samples, depth - 1).r_val()
                    }
                },
                Missed => 1.0 // The point is in direct light
            }
        }

        shade = shade / dirs.len() as f32;

        Color::init(shade, shade, shade)
    }

    fn ambient_lightning(kt: f32, ka: Color, cd: Color) -> Color {
        (cd * ka).mult(1.0 - kt)
    }

    fn calculate_fattj(light: &Light, point: Vec3) -> f32 {
        match light.kind {
            DirectionalLight => 1.0,
            _ => {
                let distance = point.distance(light.pos);
                (1.0 as f32).min(1.0 / (0.25 + 0.1 * distance + 0.01 * distance * distance))
            }
        }
    }

    fn diffuse_lightning(kt: f32, cd: Color, normal: Vec3, dj: Vec3) -> Color {
        let a: f32 = 1.0 - kt;
        let b: f32 = (0.0 as f32).max(normal.dot(dj));
        cd.mult(a * b)
    }

    fn specular_lightning(q: f32, ks: Color, normal: Vec3, dj: Vec3, v: Vec3) -> Color {
        let mut t: f32 = normal.dot(dj);
        let mut rj: Vec3 = normal.mult(t).mult(2.0);
        rj = rj - dj;
        t = rj.dot(v).max(0.0);
        ks.mult(t.powf(q))
    }

    fn direct_lightning(light: &Light, intersection: &Intersection , sj: Color, fattj: f32, num_samples: uint) -> Color {
        let point: Vec3 = intersection.point();
        let material = intersection.material();
        let kt: f32 = material.transparency;
        let cd: Color = intersection.color();
        let ks: Color = material.specular;
        let q: f32 = material.shininess * 128.0;

        let dirs: Vec<Vec3> = match light.kind {
            DirectionalLight => {
                vec!(light.dir.invert())
            },
            PointLight => {
                let mut dj = light.pos - point;
                dj.normalize();
                vec!(dj)
            },
            AreaLight => {
                let mut ori = intersection.point();

                Vec::from_fn(AREA_SAMPLES, |_| -> Vec3 {
                    let Open01(rx) = random::<Open01<f32>>();
                    let Open01(ry) = random::<Open01<f32>>();
                    let Open01(rz) = random::<Open01<f32>>();
                    let mut dx = (light.pos[0] - light.dir[0]).abs() * 0.5;
                    let mut dy = (light.pos[1] - light.dir[1]).abs() * 0.5;
                    let mut dz = (light.pos[2] - light.dir[2]).abs() * 0.5;
                    dx = dx - rx * (dx * 2.0);
                    dy = dy - ry * (dy * 2.0);
                    dz = dz - rz * (dz * 2.0);
                    let mut dj = Vec3::init(light.pos[0] + dx, light.pos[1] + dy, light.pos[2] + dz) - ori;
                    dj.normalize();
                    dj
                })
            }
        };

        let mut dir_lt = Color::new();
        let direct_light: Color = (light.intensity * sj).mult(fattj);

        for &dj in dirs.iter() {
            let normal: Vec3 = intersection.surface_normal();
            let diffuse_light: Color = RayTracer::diffuse_lightning(kt, cd, normal, dj);

            let v: Vec3 = intersection.direction().invert();
            let specular_light: Color = RayTracer::specular_lightning(q, ks, normal, dj, v);

            let mut light = direct_light * (diffuse_light + specular_light);
            light = Color::init(light.r_val() / num_samples as f32, light.g_val() / num_samples as f32, light.b_val() / num_samples as f32);
            dir_lt = dir_lt + light;
        }

        dir_lt
    }

    fn shade_intersection<'b>(scene: &'a Box<IntersectableScene<'a> + 'a>, intersection: &Intersection, num_samples: uint, depth: uint) -> Color {
        if depth <= 0 {
            return Color::new();
        }

        let material = intersection.material();
        let kt: f32 = material.transparency;
        let ks: Color = material.specular;
        let ka: Color = material.ambient;
        let cd: Color = intersection.color();

        let ambient_light: Color = RayTracer::ambient_lightning(kt, ka, cd);

        let mut direct_light: Color = Color::new();
        for light in scene.get_lights().iter() {
            let fattj = RayTracer::calculate_fattj(light, intersection.point());
            if fattj > 0.0 {
                let shadow_scalar = RayTracer::shadow_scalar(scene, light, intersection, num_samples, depth);
                direct_light = direct_light + RayTracer::direct_lightning(light, intersection, shadow_scalar, fattj, num_samples);
            }
        }

        let reflective_light = if ks.scalar() > 0.0 {
            let ray: Ray = intersection.reflective_ray();
            match scene.intersects(&ray) {
                Intersected(intersection) =>
                    ks * RayTracer::shade_intersection(scene, &intersection, num_samples, depth - 1),
                Missed => Color::new()
            }
        } else {
            Color::new()
        };

        let refractive_light = if kt > 0.0 {
            match intersection.refractive_ray() {
                Some(ray) => match scene.intersects(&ray) {
                    Intersected(intersection) =>
                        RayTracer::shade_intersection(scene, &intersection, num_samples, depth - 1).mult(kt),
                    Missed => Color::new()
                },
                None => Color::new()
            }
        } else {
            Color::new()
        };

        direct_light + ambient_light + reflective_light + refractive_light
    }

    pub fn trace_rays(&'a self) -> Image {
        match self.scene {
            Some(ref scene) => {
                let mut img = Image::new(self.width, self.height);

                for y in range(0, self.width as i32) {
                    for x in range(0, self.height as i32) {
                        let ray = self.compute_ray(x as f32, y as f32);
                        match scene.intersects(&ray) {
                            Intersected(intersection) => {
                                let color = RayTracer::shade_intersection(scene, &intersection, self.num_samples, self.depth);
                                img.set_pixel(x as uint, y as uint, color.as_pixel());
                            },
                            Missed => ()
                        }
                    }
                }
                img
            },
            None => panic!("RayTracer has not been assigned any Scene")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts;
    use std::num::Float;
    use RayTracer;
    use vec::Vec3;
    use scene::{Scene, Camera};

    fn get_raytraer<'a>() -> RayTracer<'a> {
        let mut scene = box Scene::new();
        scene.camera = Camera::new();
        scene.camera.view_dir = Vec3::init(0.0, 0.0, -1.0);
        scene.camera.ortho_up = Vec3::init(0.0, 1.0, 0.0);
        let pi: f32 = consts::PI;
        scene.camera.vertical_fov = pi / 2.0;
        let mut rt = RayTracer::init(2, 2, 2, 1);
        rt.set_scene(scene);
        rt
    }

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1.0e-6, "{} is not approximately equal to {}", a, b);
    }

    #[test]
    fn can_init_raytracer() {
        let rt = get_raytraer();
        assert_eq!(rt.width, 2);
        assert_eq!(rt.height, 2);
        assert_eq!(rt.depth, 2);
    }

    #[test]
    fn can_compute_ray() {
        let rt = get_raytraer();
        let r = rt.compute_ray(0.0, 0.0);

        assert_approx_eq(0.0, r.ori[0]);
        assert_approx_eq(0.0, r.ori[1]);
        assert_approx_eq(0.0, r.ori[2]);

        assert_approx_eq(-0.57735, r.dir[0]);
        assert_approx_eq(-0.57735, r.dir[1]);
        assert_approx_eq(-0.57735, r.dir[2]);
    }
}
