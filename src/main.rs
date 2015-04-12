extern crate cairo;
extern crate docopt;
extern crate rustc_serialize;
extern crate toml;


mod draw;
mod config;
mod types;



use config::{Args, Config, parse_config, parse_args};
use types::Point;

use std::fs::File;

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let config = parse_config(File::open("config.toml").unwrap());
    println!("{:?}", config);


    let points =vec![
                Point{ x: 0.0, y: 0.0 },
                Point{ x: 0.0, y: 1.0 },
                Point{ x: 1.0, y: 1.0 },
                Point{ x: 1.0, y: 0.0 }];
    let hull = vec![0,1,2,3];
    let inpoints = vec![0,2];
    let expoints = vec![1,3];
    draw::draw( 400, 400, &points, &hull, &inpoints, &expoints );
}
