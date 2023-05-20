// Get rid of unused errors
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let num_1: f32 = 1.11111111111111;
    println!("f32: {}", num_1 + 0.11111111111111);
    let num_2: f64 = 1.11111111111111;
    println!("f64: {}", num_2 + 0.11111111111111);
}
