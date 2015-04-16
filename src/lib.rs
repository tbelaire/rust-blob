#![feature(str_words)]
#![feature(collections)]
// Using partial_min for comparing f64s
#![feature(core)]
#[macro_use]
extern crate log;

extern crate rustc_serialize;
extern crate tau;
extern crate nalgebra as na;


pub mod blob;
pub mod config;
pub mod draw;
pub mod input;
pub mod types;

pub const EPSILON: f64 = 0.01;

