use vec::Vec3;
use ray::Ray;
use scene::material::{Material, Color};
use scene::shapes;
use scene::shapes::{BoundingBox, Shape, ShapeIntersection};

#[deriving(Show)]
pub struct Sphere {
    pub materials: Vec<Material>,
    pub origin: Vec3,
    pub radius: f32,
    pub xaxis: Vec3,
    pub xlength: f32,
    pub yaxis: Vec3,
    pub ylength: f32,
    pub zaxis: Vec3,
    pub zlength: f32
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            materials: Vec::new(),
            origin: Vec3::new(),
            radius: 0.0,
            xaxis: Vec3::new(),
            xlength: 0.0,
            yaxis: Vec3::new(),
            ylength: 0.0,
            zaxis: Vec3::new(),
            zlength: 0.0
        }
    }

    pub fn init(origin: Vec3, radius: f32) -> Sphere {
        let mut sphere = Sphere::new();
        sphere.materials = vec!(Material::new());
        sphere.origin = origin;
        sphere.radius = radius;
        sphere
    }
}

impl Shape for Sphere {
    fn get_bbox(&self) -> BoundingBox {
        let min = Vec3::init(-1.0, -1.0, -1.0);
        let max = Vec3::init(1.0, 1.0, 1.0);
        BoundingBox::init(
            min.mult(self.radius) + self.origin,
            max.mult(self.radius) + self.origin
        )
    }

    fn intersects(&self, ray: Ray) -> ShapeIntersection {
        // Transforming ray to object space
        let transformed_origin = ray.ori - self.origin;

        //Compute A, B and C coefficients
        let dest = ray.dir;
        let orig = transformed_origin;

        let a: f32 = dest.dot(dest);
        let b: f32 = 2.0 * dest.dot(orig);
        let c: f32 = orig.dot(orig) - (self.radius * self.radius);

        //Find discriminant
        let disc: f32 = b * b - 4.0 * a * c;
        // if discriminant is negative there are no real roots, so return
        // false as ray misses sphere
        if disc < 0.0 {
            return shapes::Missed;
        }

        // compute q as described above
        let dist_sqrt = disc.sqrt();
        let q = match b < 0.0 {
            true => (-b - dist_sqrt) / 2.0,
            false => (-b + dist_sqrt) / 2.0
        };

        // compute t0 and t1
        let mut t0 = q / a;
        let mut t1 = c / q;

        // make sure t0 is smaller than t1
        if t0 > t1 {
            // if t0 is bigger than t1 swap them around
            let temp = t0;
            t0 = t1;
            t1 = temp;
        }
        // if t1 is less than zero, the object is in the ray's negative direction
        // and consequently the ray misses the sphere
        if t1 < 0.0 {
            return shapes::Missed;
        }

        // if t0 is less than zero, the intersection point is at t1 else the intersection point is at t0
        match t0 < 0.0 {
            true => shapes::Hit(t1),
            false => shapes::Hit(t0)
        }
    }

    fn get_material(&self) -> Material {
        self.materials[0]
    }

    fn surface_normal(&self, _: Vec3, point: Vec3) -> Vec3 {
        let mut normal: Vec3 = point - self.origin;
        normal.normalize();
        normal
    }

    fn diffuse_color(&self, _: Vec3) -> Color {
        self.get_material().diffuse
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::shapes::sphere::Sphere;
    use scene::shapes::{Hit, Shape};

    #[test]
    fn can_init_sphere(){
        let s = Sphere::new();
        assert_eq!(s.radius, 0.0);
    }

    #[test]
    fn can_intersect_sphere() {
        let shp = Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        let ray = Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0));
        let res = shp.intersects(ray);

        match res {
            Hit(point) => assert_eq!(point, 4.0),
            _ => fail!("Ray did not intersect sphere")
        }
    }
}
