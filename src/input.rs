
use std::fs::File;
use std::io::prelude::*;

use super::types::{Point, Index};

pub fn read_points(mut f:File) -> Vec<Point> {
    let mut data_str = String::new();
    f.read_to_string(&mut data_str).unwrap();

    let data: Vec<&str> = data_str
        .words()
        .skip(1).collect();
    let points: Vec<Point> = data
        .chunks(2)
        .map(|s| Point::new(s[0].parse().unwrap(), s[1].parse().unwrap()))
        .collect();

    points
}
