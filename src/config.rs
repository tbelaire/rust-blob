
extern crate docopt;
extern crate toml;


use std::fs::File;
use std::io::prelude::*;
use self::docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: ./draw [-d] <points-file> <combs-file> <dest-prefix>

Options:
    -d, --output-directories  Output a forlder for each comb.
";

#[derive(RustcDecodable, Debug)]
pub struct Args {
    pub arg_points_file: String,
    pub arg_combs_file: String,
    pub arg_dest_prefix: String,
    pub flag_output_directories: bool,
}


#[derive(RustcDecodable, Debug)]
pub struct ConfigRun {
    pub fix_hull: bool,
    pub refine_poly: bool,
    pub rm_crossing: bool,
}
#[derive(RustcDecodable, Debug)]
pub struct ConfigImg {
    pub width : i32,
    pub height: i32,
    pub edge_buffer: f64,
}

#[derive(RustcDecodable, Debug)]
pub struct ConfigDraw {
    pub axis: bool,
    pub points: bool,
    pub polygon: bool,
    pub blob: bool,
    pub background: bool,
    pub point_radius: f64,
    pub polygon_thickness: f64,
    pub axis_thickness: f64,
}

#[derive(RustcDecodable, Debug)]
pub struct ConfigB2 {
    pub mindist_radius_factor: f64,
    pub refine_epsilon: f64,
}

#[derive(RustcDecodable, Debug)]
pub struct Config {
    pub run:  ConfigRun,
    pub img:  ConfigImg,
    pub draw: ConfigDraw,
    pub b2:   ConfigB2
}


// TODO(tbelaire) better error handling
pub fn parse_config( mut f: File ) -> Config {
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).unwrap();

    toml::decode_str(&contents).expect("Failed to parse config file")
}

pub fn parse_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}


