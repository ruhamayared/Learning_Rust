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
    //let var_name = |parameters| -> return_type {body}

    let can_vote = |age: i32| age >= 18;
    println!("Can vote: {}", can_vote(8));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1: {}", samp1);

    samp1 = 10;
    println!("samp1: {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    };
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}
