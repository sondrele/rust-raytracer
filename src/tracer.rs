use scene::{ Scene };
use vec::Vec3;
use ray::Ray;

pub mod vec;
pub mod ray;
pub mod parser;
pub mod scene;

static SCALE: f32 = 1000.0;

pub struct RayTracer {
    width: uint,
    height: uint,
    depth: uint,
    center: Vec3,
    cameraPos: Vec3,
    parallelUp: Vec3,
    parallelRight: Vec3,
    verticalFOV: f32,
    horizontalFOV: f32,
    scene: Option<Scene>
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            width: 0,
            height: 0,
            depth: 0,
            center: Vec3::new(),
            cameraPos: Vec3::new(),
            parallelUp: Vec3::new(),
            parallelRight: Vec3::new(),
            verticalFOV: 0.0,
            horizontalFOV: 0.0,
            scene: None
        }
    }

    pub fn init(width: uint, height: uint, depth: uint) -> RayTracer {
        RayTracer {
            width: width,
            height: height,
            depth: depth,
            center: Vec3::new(),
            cameraPos: Vec3::new(),
            parallelUp: Vec3::new(),
            parallelRight: Vec3::new(),
            verticalFOV: 0.0,
            horizontalFOV: 0.0,
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

        self.parallelRight = cam.viewDir.cross(cam.orthoUp);
        self.parallelUp = self.parallelRight.cross(cam.viewDir);
        self.parallelRight.normalize();
        self.parallelUp.normalize();

        self.verticalFOV = cam.verticalFOV;
        self.horizontalFOV = cam.verticalFOV * (self.width as f32 / self.height as f32);
        self.cameraPos = cam.pos;
        self.center = cam.pos + cam.viewDir.mult(SCALE);
    }

    // fn get_verticalFOV(&self) -> f32 {
    //     match self.scene {
    //         Some(ref scene) => scene.camera.verticalFOV,
    //         None => fail!("RayTracer has not been assigned any Scene")
    //     }
    // }

    // fn get_horizontalFOV(&self) -> f32 {
    //     self.get_verticalFOV() * (self.width as f32 / self.height as f32)
    // }

    fn vertical_plane(&self) -> Vec3 {
        let f = (self.verticalFOV / 2.0).tan() * SCALE;
        self.parallelUp.mult(f)
    }

    fn horizontal_plane(&self) -> Vec3 {
        let f = (self.horizontalFOV / 2.0).tan() * SCALE;
        self.parallelRight.mult(f)
    }

    fn compute_ray(&self, x: f32, y: f32) -> Ray {
        let (x, y) = (x * (1.0 / self.width as f32), y * (1.0 / self.height as f32));
        let dx = self.horizontal_plane().mult(2.0 * x - 1.0);
        let dy = self.vertical_plane().mult(2.0 * y - 1.0);
        let mut dir = self.center + dx + dy;
        dir.normalize();
        Ray::init(self.cameraPos, dir)
    }

}

#[cfg(test)]
mod tests {
    use RayTracer;
    use vec::Vec3;
    use scene::{Scene, Camera};

    fn get_raytraer() -> RayTracer {
        let mut scene = Scene::new();
        scene.camera = Camera::new();
        scene.camera.viewDir = Vec3::init(0.0, 0.0, -1.0);
        scene.camera.orthoUp = Vec3::init(0.0, 1.0, 0.0);
        let pi: f32 = Float::pi();
        scene.camera.verticalFOV = pi / 2.0;
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