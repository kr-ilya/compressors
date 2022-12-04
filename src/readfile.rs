use std::{
    fs::File,
    fs::OpenOptions,
    io::{prelude::*, BufRead, BufReader, BufWriter},
    time::Instant,
    str
};



use std::io::{self, Read, ErrorKind};

pub struct ToChunks<R> {
    reader: R,
    chunk_size: usize,
}

impl<R: BufRead> Iterator for ToChunks<R> {
    type Item = io::Result<Vec<u8>>;
    
    fn next(&mut self) -> Option<Self::Item> {

        let mut buffer = vec![0u8; self.chunk_size];
        // println!("{} {}", self.chunk_size, buffer.len());

        let mut bytes_read = 0;
        let mut flag = false;
        let mut res: Option<Self::Item> = None;

        while bytes_read != self.chunk_size && flag == false {
            res = match self.reader.read(&mut buffer[bytes_read..]) {
                Ok(n) => {
                    // println!("N= {n}");
                    if n == 0 {
                        flag = true;
                        None
                    }else {
                        bytes_read += n;
                        Some(Ok(buffer[..bytes_read].to_vec()))
                    }
                    
                },
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
                Err(e) => Some(Err(e)),
            };
        }
        if bytes_read > 0 {
            Some(Ok(buffer[..bytes_read].to_vec()))
        }else {
            res
        }
        
    }
}

pub trait IterChunks {
    type Output;
    
    fn iter_chunks(self, len: usize) -> Self::Output;
}

impl<R: BufRead> IterChunks for R {
    type Output = ToChunks<R>;
    
    fn iter_chunks(self, len: usize) -> Self::Output {
        ToChunks {
            reader: self,
            chunk_size: len,
        }
    }
}

fn write_file(file_name: &str, str_buf: &[u8]) -> std::io::Result<()> {
    let file = OpenOptions::new().append(true).create(true).open(file_name).expect("Eroro");

    let mut buffer = BufWriter::new(file);

    // buffer.write_all(str_buf)?;
    // buffer.flush()?;

    Ok(())
}