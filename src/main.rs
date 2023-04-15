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

// Result has 2 varients Ok and Err
// enum Result <T, E> {
// Ok(T)
// Err(E)}
// Where T represents the type of data the value returns
// And E represents the type of error

fn main() {
    let path: &str = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    write!(output, "Just some\nRandom words.").expect("Failed to wirte to file.");

    let input = File::open(path).unwrap();
    let bufffered = BufReader::new(input);

    for line in bufffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
