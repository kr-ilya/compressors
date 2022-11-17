use std::{
    fs::File,
    io::BufReader,
    time::Instant,
    str
};

use crate::io_controller::IterChunks;

type CodeInt = u32;

const WINDOW_SIZE: usize = 4096; // 4 KB
const BUFER_SIZE: usize = 4096;
const OUT_BUFFER_SIZE: usize = 4096; // number of codes in the output buffer


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
                        println!("{current_length}");
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
    let file = BufReader::new(File::open(file_name)?);
    let t1 = Instant::now();

    let mut codes: Vec<Code> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut full_buffer: bool = false;

    for chunk in file.iter_chunks(WINDOW_SIZE as usize) {
        match chunk {
            Ok(data) => {
                // println!("{:?}", data);
                let data_len = data.len();
                let mut viewed_len: usize = 0;
                
                while viewed_len < data_len {
                    println!("{:?}", buffer);
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
                    }

                    if full_buffer == true {
                        buffer.rotate_left(i);
                    }

                    for v in &data[data_slice_from..data_slice_to] {

                        if full_buffer == false {
                            
                            buffer.push(*v);

                            if buffer.len() == BUFER_SIZE {
                                full_buffer = true;
                                buffer.rotate_left(i-1);
                            }
                            
                        } else {
                            buffer[BUFER_SIZE-1-(i-1)] = *v;
                        }
                        
                        i = i - 1;
                    }     
                    
                    viewed_len += (encoded.length as usize) + ec;
                    codes.push(encoded);


                    if codes.len() == OUT_BUFFER_SIZE {
                        println!("Write codes to file");
                        codes.clear();
                    }

                };

                println!("{:?}", codes);

            },
            Err(e) => println!("E: {e}"),
        }
        
    }

    if codes.len() > 0 {
        println!("Write codes to file END");
        codes.clear();
    }

    let dur = t1.elapsed().as_secs_f64();

    println!("DUR: {dur}");
    Ok(())
}