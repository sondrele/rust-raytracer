extern crate bmp;

use scene::Scene;
use scene::{Intersected, Missed};
use vec::Vec3;
use ray::Ray;

use bmp::BMPimage;

pub mod vec;
pub mod ray;
pub mod parser;
pub mod scene;

static SCALE: f32 = 1000.0;

pub struct RayTracer<'a> {
    width: uint,
    height: uint,
    depth: uint,
    center: Vec3,
    camera_pos: Vec3,
    parallel_up: Vec3,
    parallel_right: Vec3,
    vertical_fov: f32,
    horizontal_fov: f32,
    scene: Option<Scene<'a>>
}

impl<'a> RayTracer<'a> {
    pub fn new() -> RayTracer<'a> {
        RayTracer {
            width: 0,
            height: 0,
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

    pub fn init(width: uint, height: uint, depth: uint) -> RayTracer<'a> {
        RayTracer {
            width: width,
            height: height,
            depth: depth,
            center: Vec3::new(),
            camera_pos: Vec3::new(),
            parallel_up: Vec3::new(),
            parallel_right: Vec3::new(),
            vertical_fov: 0.0,
            horizontal_fov: 0.0,
            scene: None
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
        self.setup_camera();
    }

    fn setup_camera(&mut self) {
        let cam = match self.scene {
            Some(ref scene) => scene.camera,
            None => fail!("RayTracer has not been assigned any Scene")
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

    pub fn trace_rays(&self) -> BMPimage {
        match self.scene {
            Some(ref scene) => {
                let mut img = BMPimage::new(self.width as i32, self.height as i32);

                for y in range(0, self.width as i32) {
                    for x in range(0, self.height as i32) {
                        let ray = self.compute_ray(x as f32, y as f32);
                        match scene.intersects(ray) {
                            Intersected(c) => img.set_pixel(x as uint, y as uint, c.as_pixel()),
                            Missed => ()
                        }
                    }
                }
                img
            },
            None => fail!("RayTracer has not been assigned any Scene")
        }
    }
}

#[cfg(test)]
mod tests {
    use RayTracer;
    use vec::Vec3;
    use scene::{Scene, Camera};

    fn get_raytraer<'a>() -> RayTracer<'a> {
        let mut scene = Scene::new();
        scene.camera = Camera::new();
        scene.camera.view_dir = Vec3::init(0.0, 0.0, -1.0);
        scene.camera.ortho_up = Vec3::init(0.0, 1.0, 0.0);
        let pi: f32 = Float::pi();
        scene.camera.vertical_fov = pi / 2.0;
        let mut rt = RayTracer::init(2, 2, 2);
        rt.set_scene(scene);
        rt
    }

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1.0e-6,
                "{} is not approximately equal to {}", a, b);
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