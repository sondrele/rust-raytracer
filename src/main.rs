#![feature(collections, env)]

extern crate rstracer;
extern crate getopts;

use std::env;
use std::str::FromStr;

use getopts::{Matches, Options};

use rstracer::scene::parser::SceneParser;
use rstracer::scene::IntersectableScene;
use rstracer::RayTracer;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief[]));
}

fn get_opt<T:FromStr>(matches: &Matches, opt: &str, default: T) -> T {
    match matches.opt_str(opt) {
        Some(opt_str) => opt_str[].parse().unwrap_or(default),
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
    "scenes/".to_string() + &name[] + ".ascii"
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let program = &args[0][];
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("b", "bvh", "Optimize scene intersection with BVH-tree");
    opts.optopt("s", "size", "The width and height of the image to be generated", "-s 500");
    opts.optopt("a", "arealight-samples", "The number of times to sample the area lights", "-a 1000");
    opts.optopt("d", "depth", "The depth of the recursion in the main loop", "-d 10");
    opts.optopt("i", "scene", "The name of a scene located in the ./scenes directory", "-i test01");
    opts.optopt("o", "out", "The name of the image to be generated", "-o image.bmp");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program[], opts);
        return;
    }

    let size = get_opt(&matches, "s", 100);
    let area_samples = get_opt(&matches, "a", 10);
    let depth = get_opt(&matches, "d", 10);
    let scene = get_scene(&matches, "test01");
    let out = get_str(&matches, "o", "img") + ".bmp";

    let mut parser = SceneParser::new(scene);
    let scene: Box<IntersectableScene> = if matches.opt_present("b") {
        Box::new(parser.parse_bvh_scene())
    } else {
        Box::new(parser.parse_scene())
    };
    let mut tracer = RayTracer::init(size, size, depth, area_samples);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    let _ = img.save(&out[]);
}
