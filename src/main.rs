mod io_controller;
use std::io;

use crate::io_controller::read_file;

fn main() {
    println!("Hello, world!");
    // _ = read_file("t100.txt");
    let s: usize = read_file("data1.txt");
    println!("enter mes");

    let mut mes = String::new();

    io::stdin().read_line(&mut mes).expect("Error read_line");
    println!("{mes}");
    println!("{s}");
    // println!("{data}");
}
