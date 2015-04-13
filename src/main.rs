extern crate rust_blob;
#[macro_use] extern crate log;
extern crate env_logger;


use rust_blob::config::{parse_config, parse_args};
use rust_blob::input::{read_points, read_combs};
use rust_blob::types::Color;
use rust_blob::blob;
use rust_blob::draw;

use std::fs::File;


/// Documenation for main
fn main() {
    env_logger::init().unwrap();
    let args = parse_args();
    info!("{:?}", args);

    let config = parse_config(File::open("config.toml").unwrap());
    info!("{:?}", config);


    info!("");
    info!("");

    let points = read_points(File::open(args.arg_points_file).unwrap());
    let combs = read_combs(File::open(args.arg_combs_file).unwrap());


    let hull_color = Color::from_hex("aaaa00");
    let inpoints_color = Color::from_hex("ff3333");
    let expoints_color = Color::from_hex("0033bb");

    for (comb_num, comb) in combs.iter().enumerate() {
        for (set_num, set) in comb.iter().enumerate() {
            use std::path::PathBuf;
            use std::fs;
            let filepath:PathBuf;
            if args.flag_output_directories {
                let filename = format!("{:02}/{:02}/{:02}.png",
                                       args.arg_dest_prefix,
                                       comb_num, set_num);
                info!("Filename: {}", filename);
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
            // Compute the in and out pointsets
            let inpoints = set;
            let expoints = (0..points.len()).filter(
                |ex| ! inpoints.iter().any(|inp| inp == ex)).collect();

            let inblob = blob::make_inblob(points.len(), inpoints);
            // Compute the polygon, and radii.
            let (hull, radii) = blob::find_hull(
                &config, &points, &inpoints, &expoints );

            info!("Hull:");
            for &ix in &hull {
                info!("({:6},{:6}) ", points[ix].x, points[ix].y);
            }
            // Draw it!
            draw::draw( &config, &points,
                        &hull, &hull_color,
                        &inpoints, &inpoints_color,
                        &expoints, &expoints_color,
                        &inblob, &radii,
                        filepath.as_path() );
        }
    }
}
