// use std::io::prelude::*;
// use std::fs::File;
// use std::fs;

use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(file_name: &str) -> usize {
//     let mut file = File::open(file_name).expect("Unable to open the file");
//     let mut data = String::new();
//     file.read_to_string(&mut data).expect("Unable to read the file");

//     data

    let file = File::open(file_name).expect("Unable to open the file");
    let mut data = String::new();
    let mut br = BufReader::new(file);
    br.read_to_string(&mut data).expect("Unable to read the file");

    // println!("{}", data.len());
    println!("{}", &data[data.len()-30..]);

    data.len()
}
