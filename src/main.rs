#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val)
    }

    arr_it.into_iter();

    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next());
}
