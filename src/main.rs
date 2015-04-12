#![feature(str_words)]
extern crate rustc_serialize;
extern crate tau;


mod blob;
mod config;
mod draw;
mod input;
mod types;

const EPSILON: f64 = 0.01;



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

    for (comb_num, comb) in combs.iter().enumerate() {
        for (set_num, set) in comb.iter().enumerate() {
            use std::path::PathBuf;
            use std::fs;
            let hull = blob::giftwrap(&points, &set);
            let inpoints = set;
            let expoints = (0..points.len()).filter(
                |ex| ! inpoints.iter().any(|inp| inp == ex)).collect();
            let filepath:PathBuf;
            if args.flag_output_directories {
                let filename = format!("{:02}/{:02}/{:02}.png",
                                       args.arg_dest_prefix,
                                       comb_num, set_num);
                println!("Filename: {}", filename);
                filepath = PathBuf::from(filename);
                match filepath.parent() {
                    Some(parent) => fs::create_dir_all(parent).unwrap(),
                    _ => (),
                }
            } else {
                let filename = format!("{:02}_{:02}_{:02}.png",
                                       args.arg_dest_prefix,
                                       comb_num, set_num);
                filepath = PathBuf::from(filename);

            }
            draw::draw( &config, &points, &hull, &inpoints, &expoints, filepath.as_path() );
        }
    }
}
