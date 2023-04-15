#![allow(unused)]

use rand::Rng;
// use std::arch::aarch64::ST;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy!");
    println!("Message: {}", name)
}

fn main() {
    let mut str1 = String::from("Ruhama");
    change_string(&mut str1)
}
