extern crate rstracer;
extern crate getopts;

use getopts::{Matches, optopt,optflag,getopts,OptGroup, usage};
use std::os;

use rstracer::RayTracer;
use rstracer::parser::SceneParser;

fn parse_command_line(program: &str, args: &[String], opts: &[OptGroup]) -> Matches {
    match getopts(args, opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", usage(program, opts));
            fail!(f.to_string())
        }
    }
}

fn get_opt(matches: &Matches, opt: &str, default: uint) -> uint {
    match matches.opt_str(opt) {
        Some(opt_str) => from_str::<uint>(opt_str.as_slice()).unwrap_or(default),
        None => default
    }
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].as_slice();
    let opts = [
        optflag("h", "help", "Print this help menu"),
        optopt("s", "size", "The width and height of the image to be generated", "-s 500"),
        optopt("d", "depth", "The depth of the recursion in the main loop", "-d 10")
    ];
    let matches = parse_command_line(program, args.tail(), opts);
    if matches.opt_present("h") {
        println!("{}", usage(program, opts));
        return;
    }

    let size = get_opt(&matches, "s", 100);
    let depth = get_opt(&matches, "d", 10);

    let mut parser = SceneParser::new("scenes/test06.ascii".to_string());
    let scene = parser.parse_scene();
    let mut tracer = RayTracer::init(size, size, depth);
    tracer.set_scene(scene);
    let img = tracer.trace_rays();
    img.save("img.bmp");
}
