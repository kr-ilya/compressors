use std::env;
use std::time::Instant;

extern crate compressors;
use compressors::algorithms;

fn print_help() {
    println!("Compressors");
    println!("USAGE: ./compressors <file_path> <method> <mode>");
    println!("`<file_path>` - file path");
    println!("`<method>` - compression method");
    println!("`<mode>` - mode");
    println!("");
    println!("Available methods:");
    println!("- LZ77");
    println!("Available modes:");
    println!("p - pack");
    println!("u - unpack");
    println!("");
    println!("Example:");
    println!("./compressors in.txt LZ77 p");
}

/// # Compressors
/// USAGE: ./compressors <file_path> <method> <mode>
/// * `<file_path>` - file path
/// * `<method>` - compression method
/// * `<mode>` - mode (p - pack, u - unpack)
///
/// Available methods:
/// * LZ77
///
/// Available modes:
/// * p - pack
/// * u - unpack
///
/// Example:
/// ./compressors in.txt LZ77 p
///

fn main() {
    let mut methods: Vec<String> = vec![String::new(); 1];
    methods[0] = "lz77".to_string();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        if args.len() == 2 && &args[1] == "help" {
            print_help();
        } else {
            println!("Wrong cmd arguments");
        }
        return;
    }

    let file_path = &args[1];
    let method = &args[2].to_lowercase();
    let mode = &args[3];

    if !methods.iter().any(|i| i == method) {
        println!("Wrong method.");
        println!("'./compressors help' list available methods");
        return;
    }

    if mode != "p" && mode != "u" {
        println!("Wrong mode.");
        println!("'./compressors help' list available modes");
        return;
    }

    let t1 = Instant::now();

    if method == "lz77" {
        if mode == "p" {
            algorithms::lz77::compress(&file_path, 0).unwrap();
        } else {
            algorithms::lz77::decompress(&file_path, &file_path[..file_path.len() - 5]).unwrap();
        }
    }

    let dur = t1.elapsed().as_secs_f64();

    println!("DUR: {dur}");
}
