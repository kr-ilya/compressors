use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{collections::HashMap, fs::File, fs::OpenOptions, io, io::BufWriter, io::Write, str};

use crate::io_tools::read_file;

type CodeInt = u32;

#[derive(Debug)]
struct Code {
    offset: CodeInt,
    next: u8,
}

fn write_codes(file_out: &mut BufWriter<File>, codes: &Vec<Code>) {
    let mut out_buffer: Vec<u8> = Vec::new();

    for v in codes {
        out_buffer.write_u32::<LittleEndian>(v.offset).unwrap();
        out_buffer.push(v.next);
    }

    let a: &[u8] = &out_buffer;
    file_out.write_all(a).unwrap();
    file_out.flush().unwrap();

    out_buffer.clear();
}

/// File compression
///
/// # Arguments:
/// * `file_name` - Name of input file
pub fn compress(file_name: &str) -> Result<&str, io::Error> {
    let mut codes: Vec<Code> = Vec::new();
    let mut dict: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut data: Vec<u8> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    let mut last_index: usize = 0;

    read_file(file_name, &mut data).unwrap();

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name.to_owned() + ".lz78")?;

    let mut out_writer = BufWriter::new(file_out);

    for v in &data {
        buf.push(*v);

        let tmp = dict.get(&buf);
        if tmp == None {
            codes.push(Code {
                offset: last_index as CodeInt,
                next: *v,
            });

            dict.insert(buf.clone(), dict.len() + 1);
            buf.clear();
            last_index = 0;
        } else {
            last_index = *tmp.unwrap();
        }
    }

    if !buf.is_empty() {
        codes.push(Code {
            offset: last_index as CodeInt,
            next: 0,
        });
    }

    write_codes(&mut out_writer, &codes);

    Ok("Ok")
}

/// File decompression
///
/// # Arguments:
///
/// * `file_name` - Name of the compressed file
/// * `out_file_name` - Output file name
pub fn decompress<'a>(file_name: &'a str, out_file_name: &'a str) -> Result<&'a str, io::Error> {
    let mut result: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    let mut dict: Vec<Vec<u8>> = Vec::new();

    let data_len = read_file(file_name, &mut data).unwrap();

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_file_name)?;

    let mut out_writer = BufWriter::new(file_out);
    let mut viewed_len: usize = 0;

    while viewed_len < data_len {
        let offset: CodeInt = (&data[viewed_len..]).read_u32::<LittleEndian>().unwrap();
        let next: u8 = data[viewed_len + 4];

        let mut tmp: Vec<u8> = Vec::new();

        if offset != 0 {
            tmp.extend(dict[(offset - 1) as usize].clone());
        }
        tmp.push(next);

        dict.push(tmp.clone());
        result.append(&mut tmp);

        viewed_len += 5;
    }

    if result[result.len() - 1] == 0 {
        result.pop();
    }

    let a: &[u8] = &result;
    out_writer.write_all(a)?;
    out_writer.flush()?;

    Ok("Ok")
}
