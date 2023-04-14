#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    let mut loop_idx: usize = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
}
