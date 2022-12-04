use std::{
    fs::File,
    io::BufReader,
    io::BufWriter,
    io::Write,
    io::Read,
    fs::OpenOptions,
    str,
    cmp,
};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use crate::readfile::IterChunks;

type CodeInt = u32;

const WINDOW_SIZE: usize = 10; // 4 KB
const BUFER_SIZE: usize = 4096;
const OUT_BUFFER_SIZE: usize = 4096; // number of CODES in the output buffer
const COMPRESSED_CHUNK_SIZE: usize = 456 * 9; // read number of BYTES (a multiple of 9)


#[derive(Debug)]
struct Code {
    literal: u8,
    offset: CodeInt,
    length: CodeInt,
}


fn find_match(data: &[u8], buf: &Vec<u8>) -> Code {
    
    let buf_len = buf.len();
    let data_len = data.len();
    
    if buf_len == 0 {
        Code {
            literal: data[0],
            offset: 0,
            length: 0
        }
    }else{
        
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
                }else{
                    current_byte = data[ow_length as usize];
                    ow_flag = true;
                }


                if current_byte != data[current_length as usize] {
                    break;
                }else {
                    current_length += 1;

                    if ow_flag {
                        ow_length += 1;
                    }
                    
                    if current_length as usize == data_len {
                        // println!("{current_length}");
                        break;
                    }
                }                
            }
            
            if current_length > best_length {
                best_length = current_length;
                best_offset = i as u32;
            }
        }

        if best_length as usize != data_len {
            next_byte = data[best_length as usize];
        }else {
            next_byte = 0;
        }

        Code {
            literal: next_byte,
            offset: best_offset,
            length: best_length
        }
    }
}

pub fn compress(file_name: &str) -> std::io::Result<()>{
    let file_in = BufReader::new(File::open(file_name)?);

    let file_out = OpenOptions::new().write(true).create(true).truncate(true).open("out.txt").expect("Eroro");
    let mut out_writer = BufWriter::new(file_out);

    let mut codes: Vec<Code> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut full_buffer: bool = false;
    let mut nt = 0;
    let mut st = 0;
    let mut cn = 0;
    for chunk in file_in.iter_chunks(WINDOW_SIZE as usize) {
        match chunk {
            Ok(data) => {
                cn += 1;
                // println!("{:?}", data);
                let data_len = data.len();
                let mut viewed_len: usize = 0;

                
                let mut out_buffer: Vec<u8> = Vec::new();
                
                while viewed_len < data_len {
                    // println!("{:?}", buffer);

                    //  TO DO
                    //  ПРОЧИТАТЬ ССЛЕЮЩУИЙ ЧАНК, ЧТОБЫ ПРАВИЛЬНО ЗАКОДИРОВАТЬ
                    // 


                    let encoded: Code = find_match(&data[viewed_len..], &buffer);

                    let mut ec = 1;
                    if viewed_len + (encoded.length as usize) + 1 > data_len {
                        ec = 0;
                    }

                    let mut i = (encoded.length as usize) + ec;
                    let mut data_slice_from = viewed_len;
                    let data_slice_to = viewed_len + (encoded.length as usize) + ec;

                    if i > BUFER_SIZE {
                        data_slice_from = data_slice_to - BUFER_SIZE;
                        full_buffer = true;
                        i = BUFER_SIZE-1 + ec;
                    }

                    if full_buffer == true {
                        buffer.rotate_left(cmp::min(i-1, BUFER_SIZE));
                    }

                    for v in &data[data_slice_from..data_slice_to] {

                        if full_buffer == false {
                            
                            buffer.push(*v);

                            if buffer.len() == BUFER_SIZE {
                                full_buffer = true;
                                // TO DO
                                // change to pointer
                                buffer.rotate_left(cmp::min(i-1, BUFER_SIZE));
                            }
                            
                        } else {
                            buffer[BUFER_SIZE-1-(i-1)] = *v;
                        }
                        
                        i = i - 1;
                    }     
                    
                    viewed_len += (encoded.length as usize) + ec;
                    // println!("VL {viewed_len}");
                    codes.push(encoded);
                    nt += 1;

                    if codes.len() == OUT_BUFFER_SIZE {
                        // println!("Write codes to file");

                        for v in &codes {
                            st += 1;
                            out_buffer.push(v.literal);
                            out_buffer.write_u32::<LittleEndian>(v.offset).unwrap();
                            out_buffer.write_u32::<LittleEndian>(v.length).unwrap();
                        }

                        let a: &[u8] = &out_buffer;
                        out_writer.write_all(a)?;
                        out_writer.flush()?;
                        
                        out_buffer.clear();
                        codes.clear();
                    }

                };

                // println!("{:?}", codes);

                
                for v in &codes {
                    st += 1;
                    out_buffer.push(v.literal);
                    out_buffer.write_u32::<LittleEndian>(v.offset).unwrap();
                    out_buffer.write_u32::<LittleEndian>(v.length).unwrap();
                }

                // println!("{:?}", out_buffer);
            
                // WRITE
                let a: &[u8] = &out_buffer;
                out_writer.write_all(a)?;
                out_writer.flush()?;

                out_buffer.clear();
                codes.clear();

            },
            Err(e) => println!("E: {e}"),
        }
        
    }

    if codes.len() > 0 {
        // println!("Write END codes to file");
        codes.clear();
    }
    // 863 372696 848
    println!("{nt} {st} {cn}");
    Ok(())
}

// TO DO
// change buffers to array[], prmf

pub fn decompress(file_name: &str) -> std::io::Result<()> {
    let file_in = BufReader::new(File::open(file_name)?);

    let file_out = OpenOptions::new().write(true).create(true).truncate(true).open("decompressed.txt").expect("Eroro");
    let mut out_writer = BufWriter::new(file_out);

    let mut result: Vec<u8> = Vec::new();
    let mut full_buffer: bool = false;
    let mut buf_len = 0;
    let mut sss = 0;
    let mut ch = 0;
    for chunk in file_in.iter_chunks(COMPRESSED_CHUNK_SIZE as usize) {
        match chunk {
            Ok(data) =>{

                println!("{:?}", data);

                ch += 1;
                let data_len = data.len();
                let mut viewed_len: usize = 0;

                while(viewed_len < data_len) {
                    // println!("A1 {}", data_len);
                    // println!("{} {:?} {:?}", data[viewed_len], &data[viewed_len+1..viewed_len+5], &data[viewed_len+5..viewed_len+9]);

                    // println!("{viewed_len}");
                    // let literal: u8 = data[viewed_len];
                    // let offset:CodeInt = (&data[viewed_len+1..]).read_u32::<LittleEndian>().unwrap();
                    // let length:CodeInt = (&data[viewed_len+5..]).read_u32::<LittleEndian>().unwrap();
                    sss += 1;
                    // println!("a {} {} {}", literal, offset, length);
                    
                    // let mut i = (length + 1) as usize;

                    
                    // if i + buf_len-1 > BUFER_SIZE {
                    //     // WRITE TO FILE
                    //     let a: &[u8] = &result;
                    //     out_writer.write_all(a)?;
                    //     out_writer.flush()?;


                    //     // println!("WR");
                    //     // println!("{:?}", result);
                    //     full_buffer = true;
                    //     buf_len = 0;
                    // }

                    // if length == 0 {
                    //     if full_buffer == true {
                    //         result.rotate_left(1);
                    //         result[BUFER_SIZE-1] = literal;
                    //     } else {
                    //         result.push(literal);
                    //     }
                    //     buf_len += 1;
                    // }else{
                    //     let res_len = result.len();

                    //     // check length > len-offset
                    //     let mut tmpstr: Vec<u8> = Vec::new();
                    //     let mut tml_len = length as usize;

                    //     while tml_len != 0 {
                    //         let tl = cmp::min(res_len - (offset as usize), tml_len as usize);
                    //         tmpstr.extend_from_slice(&result[(offset as usize)..(offset as usize+tl)]);
                    //         tml_len -= tl;
                    //     }

                        
                    //     buf_len += i;

                    //     if full_buffer == true {
                    //         result.rotate_left(cmp::min(i, BUFER_SIZE));
                    //     }

                    //     let mut ts = 0;
                    //     while i != 1 {
                    //         let mut j = cmp::min(BUFER_SIZE-1, i);
                    //         // println!("J = {j} {i}");
                    //         while j != 1 {
                    //             if full_buffer == true {
                    //                 result[BUFER_SIZE-1 - j] = tmpstr[ts];                                   
                    //             }else {
                    //                 result.push(tmpstr[ts]);
                    //             }
                    //             i -= 1;
                    //             j -= 1;
                    //             ts += 1;
                    //         }
                    //         // println!("pI = {i} {j}");
                    //         if i > 1 {
                    //             // println!("I = {i} {j}");
                    //             let a: &[u8] = &result;
                    //             out_writer.write_all(a)?;
                    //             out_writer.flush()?;
                    //             result.rotate_left(cmp::min(i, BUFER_SIZE));
                    //         }
                    //     }

                    //     if literal != 0 {
                    //         if full_buffer == true {
                    //             result[BUFER_SIZE-1] = literal;
                    //         }else{
                    //             result.push(literal);
                    //         }
                    //     }

                    // }

                    viewed_len += 9;
                }
                
            },
            Err(e) => println!("Decompress error"),
        }
    }
    
    println!("s {sss}");
    println!("c {ch}");
    println!("L WR");
    // println!("{:?}", result);

    let a: &[u8] = &result;
    out_writer.write_all(a)?;
    out_writer.flush()?;
    

    Ok(())
}