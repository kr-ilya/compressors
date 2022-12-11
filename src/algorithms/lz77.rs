use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{cmp, fs::File, fs::OpenOptions, io, io::BufWriter, io::Write, str};

use crate::io_tools::read_file;

type CodeInt = u32;

const WINDOW_SIZE: usize = 32768; // 32 KB
const BUFFER_SIZE: usize = 32768;
const OUT_BUFFER_SIZE: usize = 450000; // number of CODES in the output buffer

#[derive(Debug)]
struct Code {
    literal: u8,
    offset: CodeInt,
    length: CodeInt,
}

fn find_match(data: &[u8], buf: &Vec<u8>, compression_lvl: usize) -> Code {
    let buf_len = buf.len();
    let data_len = data.len();
    let end_len = cmp::min(data_len, WINDOW_SIZE);

    if buf_len == 0 {
        Code {
            literal: data[0],
            offset: 0,
            length: 0,
        }
    } else {
        let mut best_length: CodeInt = 0;
        let mut best_offset: CodeInt = 0;
        let mut current_byte: u8;
        let next_byte: u8;

        for i in 0..buf_len {
            let mut current_length: CodeInt = 0;
            let mut ow_length: CodeInt = 0;
            let mut ow_flag: bool = false;

            loop {
                if i + (current_length as usize) < buf_len {
                    current_byte = buf[i + (current_length as usize)];
                } else {
                    current_byte = data[ow_length as usize];
                    ow_flag = true;
                }

                if current_byte != data[current_length as usize] {
                    break;
                } else {
                    current_length += 1;

                    if ow_flag {
                        ow_length += 1;
                    }

                    if current_length as usize == end_len {
                        break;
                    }
                }
            }

            if current_length > best_length {
                best_length = current_length;
                best_offset = i as u32;

                if compression_lvl == 1 {
                    break;
                }
            }
        }

        if best_length as usize == data_len {
            next_byte = 0;
        } else {
            if (best_length as usize) == WINDOW_SIZE {
                best_length -= 1;
            }
            next_byte = data[best_length as usize];
        }

        Code {
            literal: next_byte,
            offset: best_offset,
            length: best_length,
        }
    }
}

fn write_codes(file_out: &mut BufWriter<File>, codes: &Vec<Code>) {
    let mut out_buffer: Vec<u8> = Vec::new();

    for v in codes {
        out_buffer.push(v.literal);
        out_buffer.write_u32::<LittleEndian>(v.offset).unwrap();
        out_buffer.write_u32::<LittleEndian>(v.length).unwrap();
    }

    let a: &[u8] = &out_buffer;
    file_out.write_all(a).unwrap();
    file_out.flush().unwrap();

    out_buffer.clear();
}

/// File compression
///
/// # Arguments
///
/// * `file_name` - Name of input file
/// * `compression_lvl` - Compression level
///
/// # Compression level:
/// - 0 - Slow, best compression
/// - 1 - Fast, standart compression
pub fn compress(file_name: &str, compression_lvl: usize) -> Result<&str, io::Error> {
    let mut codes: Vec<Code> = Vec::new();
    let mut buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
    let mut full_buffer: bool = false;
    let mut data: Vec<u8> = Vec::new();

    let data_len = read_file(file_name, &mut data).unwrap();

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name.to_owned() + ".lz77")?;

    let mut out_writer = BufWriter::new(file_out);

    let mut viewed_len: usize = 0;

    while viewed_len < data_len {
        let encoded: Code = find_match(&data[viewed_len..], &buffer, compression_lvl);

        let mut ec = 1;
        if viewed_len + (encoded.length as usize) + 1 > data_len {
            ec = 0;
        }

        let mut i = (encoded.length as usize) + ec;

        let mut data_slice_from = viewed_len;
        let data_slice_to = viewed_len + (encoded.length as usize) + ec;

        if i > BUFFER_SIZE {
            data_slice_from = data_slice_to - BUFFER_SIZE;
            i = BUFFER_SIZE;
        }

        if full_buffer == true && i < BUFFER_SIZE {
            buffer.rotate_left(i);
        }

        for v in &data[data_slice_from..data_slice_to] {
            if full_buffer == false {
                buffer.push(*v);
                i = i - 1;

                if buffer.len() == BUFFER_SIZE {
                    full_buffer = true;
                    buffer.rotate_left(cmp::min(i, BUFFER_SIZE));
                }
            } else {
                buffer[BUFFER_SIZE - 1 - (i - 1)] = *v;
                i = i - 1;
            }
        }

        viewed_len += (encoded.length as usize) + ec;
        codes.push(encoded);

        if codes.len() == OUT_BUFFER_SIZE {
            write_codes(&mut out_writer, &codes);
            codes.clear();
        }
    }

    write_codes(&mut out_writer, &codes);
    codes.clear();

    Ok("Ok")
}

/// File decompression
///
/// # Arguments
///
/// * `file_name` - Name of the compressed file
/// * `out_file_name` - Output file name
pub fn decompress<'a>(file_name: &'a str, out_file_name: &'a str) -> Result<&'a str, io::Error> {
    let mut result: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = Vec::new();

    let data_len = read_file(file_name, &mut data).unwrap();

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_file_name)?;
    let mut out_writer = BufWriter::new(file_out);

    let mut viewed_len: usize = 0;

    while viewed_len < data_len {
        let literal: u8 = data[viewed_len];
        let offset: CodeInt = (&data[viewed_len + 1..])
            .read_u32::<LittleEndian>()
            .unwrap();
        let length: CodeInt = (&data[viewed_len + 5..])
            .read_u32::<LittleEndian>()
            .unwrap();

        if length == 0 {
            result.push(literal);
        } else {
            let res_len = result.len();

            let mut tmpstr: Vec<u8> = Vec::new();

            let mut sf = 0;
            if res_len > BUFFER_SIZE {
                sf = res_len - BUFFER_SIZE;
            }

            let mut nt = length as usize;

            while nt > 0 {
                let st = cmp::min(res_len - (sf + offset as usize), nt);
                tmpstr
                    .extend_from_slice(&result[(sf + offset as usize)..sf + offset as usize + st]);
                nt -= st;
            }

            for v in &tmpstr {
                result.push(*v);
            }

            if literal != 0 {
                result.push(literal);
            }
        }

        viewed_len += 9;
    }

    let a: &[u8] = &result;
    out_writer.write_all(a)?;
    out_writer.flush()?;

    Ok("Ok")
}
