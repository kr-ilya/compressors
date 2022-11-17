// use std::io::Read;
// use std::fs::File;

mod io_controller;
mod algorithms;

fn main() {
    println!("Hello, world!");

    _ = algorithms::lz77::compress("data1.txt");

    // _ = read_file("t100.txt");
    // _ = read_file("data1.txt");

    // let mut b = std::io::BufReader::new(File::open("data1.txt").expect("asd"));
    // let mut buf = vec![0u8; 0x40];
    // let n = b.read(&mut buf).expect("qwe");
    // println!("{n}");
    // println!("{buf:?}");
    // println!("{:?}", &buf[..n]);

    // let n = b.read(&mut buf).expect("qwe");
    // println!("{n}");
    // println!("{buf:?}");
    // println!("{:?}", &buf[..n]);


    println!("enter mes");

    // let mut mes = String::new();

    // io::stdin().read_line(&mut mes).expect("Error read_line");
    // println!("{mes}");
    // println!("{data}");
}
