#![feature(str_words)]
#![feature(collections)]
extern crate rustc_serialize;
extern crate tau;
extern crate nalgebra as na;


pub mod blob;
pub mod config;
pub mod draw;
pub mod input;
pub mod types;

pub const EPSILON: f64 = 0.01;

