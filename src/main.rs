extern crate rstracer;
extern crate getopts;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;

use rstracer::RayTracer;
use rstracer::parser::SceneParser;

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-h --help\tUsage");
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = [
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    let mut parser = SceneParser::new("scenes/test06.ascii".to_string());
    let scene = parser.parse_scene();
    let mut tracer = RayTracer::init(250, 250, 10);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    img.save("img.bmp");
}
