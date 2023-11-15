#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // println!("Whats your name?");
    // let mut name = String::new();
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Couldnt read the line");
    // println!("Hello, {}, Nice to meet you!", name.trim_end());
    // let age = "43";
    // let mut age: u32 = age.trim().parse().expect("Age was not parsed ((((");
    // age = age + 1;
    // println!("{}", age);
    // println!("{}", u8::MAX);
    // let random = rand::thread_rng().gen_range(1..101);
    // println!("{}", random);
    // let mut age = String::new();
    // println!("Whats your age?");
    // io::stdin()
    //     .read_line(&mut age)
    //     .expect("Couldnt find age(((");
    // let mut age: u16 = age.trim_end().parse().expect("cannt parse");
    // match age {
    //     0..=18 => println!("ragac veraris kargad"),
    //     19..=u16::MAX => println!("aq ukve norm"),
    // }
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 8];
    println!("First item is: {}", arr_1[0]);
    println!("Length of array is: {}", arr_1.len());
}
