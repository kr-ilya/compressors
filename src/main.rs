// use std::io::Read;
// use std::fs::File;
use std::time::Instant;

mod readfile;
mod algorithms;

fn main() {
    println!("Hello, world!");

    // _ = algorithms::lz77::compress("data1.txt");

    let t1 = Instant::now();
    _ = algorithms::lz77::compress("data1.txt");
    // _ = algorithms::lz77::compress("t2.txt");


    // _ = algorithms::lz77::decompress("out.txt");


    let dur = t1.elapsed().as_secs_f64();

    println!("DUR: {dur}");

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


    println!("end main");

    // let mut mes = String::new();

    // io::stdin().read_line(&mut mes).expect("Error read_line");
    // println!("{mes}");
    // println!("{data}");
}
