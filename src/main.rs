extern crate rstracer;

use rstracer::RayTracer;
use rstracer::parser::SceneParser;

#[allow(dead_code)]
fn main() {
    let mut parser = SceneParser::new("scenes/test06.ascii".to_string());
    let scene = parser.parse_scene();
    let mut tracer = RayTracer::init(250, 250, 10);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    img.save("img.bmp");
}
