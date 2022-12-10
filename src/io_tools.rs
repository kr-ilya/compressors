use std::{fs::File, io::BufReader};

use std::io::Read;

pub fn read_file(file_name: &str, data: &mut Vec<u8>) -> Result<usize, std::io::Error> {
    let mut file = BufReader::new(File::open(file_name)?);
    file.read_to_end(data)
}
