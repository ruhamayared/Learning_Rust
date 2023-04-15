#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob: Customer = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main Street")
}
