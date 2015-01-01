extern crate rstracer;
extern crate getopts;

use std::os;
use std::str::FromStr;
use getopts::{Matches, optopt, optflag, getopts, OptGroup};

use rstracer::scene::parser::SceneParser;
use rstracer::scene::IntersectableScene;
use rstracer::RayTracer;

fn parse_command_line(program: &str, args: &[String], opts: &[OptGroup]) -> Matches {
    match getopts(args, opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", getopts::usage(program, opts));
            panic!(f.to_string())
        }
    }
}

fn get_opt<T:FromStr>(matches: &Matches, opt: &str, default: T) -> T {
    match matches.opt_str(opt) {
        Some(opt_str) => opt_str.as_slice().parse().unwrap_or(default),
        None => default
    }
}

fn get_str(matches: &Matches, opt: &str, default: &str) -> String {
    match matches.opt_str(opt) {
        Some(opt_str) => opt_str,
        None => default.to_string()
    }
}

fn get_scene(matches: &Matches, default: &str) -> String {
    let name = get_str(matches, "i", default);
    String::from_str("scenes/") + name.as_slice() + ".ascii"
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].as_slice();
    let opts = [
        optflag("h", "help", "Print this help menu"),
        optflag("b", "bvh", "Optimize scene intersection with BVH-tree"),
        optopt("s", "size", "The width and height of the image to be generated", "-s 500"),
        optopt("a", "arealight-samples", "The number of times to sample the area lights", "-a 1000"),
        optopt("d", "depth", "The depth of the recursion in the main loop", "-d 10"),
        optopt("i", "scene", "The name of a scene located in the ./scenes directory", "-i test01"),
        optopt("o", "out", "The name of the image to be generated", "-o image.bmp")
    ];
    let matches = parse_command_line(program, args.tail(), &opts);
    if matches.opt_present("h") {
        println!("{}", getopts::usage(program, &opts));
        return;
    }

    let size = get_opt(&matches, "s", 100);
    let area_samples = get_opt(&matches, "a", 10);
    let depth = get_opt(&matches, "d", 10);
    let scene = get_scene(&matches, "test01");
    let out = get_str(&matches, "o", "img") + ".bmp";

    let mut parser = SceneParser::new(scene);
    let scene: Box<IntersectableScene> = if matches.opt_present("b") {
        box parser.parse_bvh_scene()
    } else {
        box parser.parse_scene()
    };
    let mut tracer = RayTracer::init(size, size, depth, area_samples);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    img.save(out.as_slice());
}
