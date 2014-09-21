extern crate tracer;

use tracer::RayTracer;
use tracer::parser::SceneParser;

fn main() {
    let mut parser = SceneParser::new("scenes/test01.ascii".to_string());
    let scene = parser.parse_scene();
    let mut tracer = RayTracer::init(500, 500, 10);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    img.save("img.bmp");
}
