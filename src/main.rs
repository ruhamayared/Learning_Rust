#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = 20;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday!")
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday!")
    } else if age >= 65 {
        println!("Important Birthday!")
    } else {
        println!("Not an important birthday 😪.")
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can Vote: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday!"),
        21 | 50 => println!("Important Birthday!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("Not an important birthday 😪."),
    };
}
