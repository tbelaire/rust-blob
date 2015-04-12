#![feature(str_words)]
extern crate cairo;
extern crate docopt;
extern crate nalgebra;
extern crate rustc_serialize;
extern crate toml;


mod config;
mod draw;
mod input;
mod types;



use config::{parse_config, parse_args};
use input::{read_points, read_combs};

use std::fs::File;

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let config = parse_config(File::open("config.toml").unwrap());
    println!("{:?}", config);

    let points = read_points(File::open(args.arg_points_file).unwrap());
    let combs = read_combs(File::open(args.arg_combs_file).unwrap());

    let hull = vec![0,1,2,3];
    let inpoints = vec![0,2];
    let expoints = vec![1,3];
    draw::draw( &config, &points, &hull, &inpoints, &expoints );
}
