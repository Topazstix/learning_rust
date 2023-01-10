#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // PART 1
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    
    println!("Hello {}! {}", name.trim_end(), greeting);

    //PART 2
    const ONE_MILL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasnt assigned a number");
    // ^^ code above takes age original string, converts it into a new var age
    // and hands the errors within the actual assignment
    age += 1;
    
    println!("Im {} and i want ${}", age, ONE_MILL);

    //PART 3
    // data types
    // unsigned ints: u8, u16, u32, u64, u128, usize
    // signed ints:  i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);


}
