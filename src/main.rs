// use std::io::Read;
// use std::fs::File;
use std::time::Instant;

extern crate compressors;
use compressors::algorithms;

fn main() {
    println!("Hello, world!");

    let t1 = Instant::now();

    algorithms::lz77::compress("data1.txt", 0).unwrap();
    // algorithms::lz77::decompress("data1.txt.lz77", "decompressed.txt").unwrap();

    let dur = t1.elapsed().as_secs_f64();

    println!("DUR: {dur}");

    println!("end main");
}
