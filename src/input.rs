
use std::fs::File;
use std::io::prelude::*;

use types::{Point, Index, Comb};

pub fn read_points(mut f:File) -> Vec<Point> {
    let mut data_str = String::new();
    f.read_to_string(&mut data_str).unwrap();

    let data: Vec<&str> = data_str
        .split(char::is_whitespace)
        .skip(1).collect();
    let points: Vec<Point> = data
        .chunks(2)
        .filter(|s| s.len() == 2)
        .map(|s| {
            Point::new(s[0].parse().unwrap(), s[1].parse().unwrap())
        })
        .collect();

    points
}

pub fn read_combs(mut f:File) -> Vec<Comb> {
    let mut data_str = String::new();
    f.read_to_string(&mut data_str).unwrap();

    let mut it = data_str.lines();
    let mut combs: Vec<Comb> = Vec::new();
    loop {
        let comb_size: i32 = match it.next() {
            None => break,
            Some(comb_size_str) => comb_size_str.trim().parse().unwrap()
        };
        let mut sets: Vec<Vec<Index>> = Vec::new(); // Also Comb
        for _ in 0..comb_size {
            let line = it.next().expect("Comb cut short");

            let set: Vec<Index> = line.split(char::is_whitespace).skip(2) // Dropping the size
                .map(|s| s.parse().unwrap())
                .collect();
            sets.push(set);
        }
        let _ = it.next(); // Dropping the extra value
        combs.push(sets);

    }
    combs
}
