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

        match self.reader.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    return None
                }else {
                    return Some(Ok(buffer[..n].to_vec()))
                }
                
            },
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
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

// pub fn read_file(file_name: &str) -> ToChunks<BufReader<File>> {
//     // let file = BufReader::new(File::open(file_name)?);
//     // let mut s: usize = 0;
//     // let t1 = Instant::now();
//     // const CHUNK_SIZE: usize = 4096; // 4 KB
//     // for chunk in file.iter_chunks(CHUNK_SIZE) {
//     //     println!("{:?}", chunk);
//     //     match chunk {
//     //         Ok(data) => {
//     //             s += data.len();
//     //         },
//     //         Err(e) => println!("E: {e}"),
//     //     }
        
//     // }

//     // let dur = t1.elapsed().as_secs_f64();

//     // println!("DUR: {dur}");
//     // println!("S = {s}");

//     // Ok(())


//     let file = BufReader::new(File::open(file_name)?);

//     const CHUNK_SIZE: usize = 4096; // 4 KB
//     file.iter_chunks(CHUNK_SIZE)
// }


// pub fn read_file(file_name: &str) -> std::io::Result<()> {
//     const CAP: usize = 4_194_304; // 4 MB
//     let file = File::open(file_name).expect("Error read file");
//     let mut reader = BufReader::with_capacity(CAP, file);
//     let t1 = Instant::now();
//     let mut s: usize = 0;
//     loop {

//         let buffer = reader.fill_buf().unwrap();
//         // println!("{buffer:?}");

//         // let s = match str::from_utf8(buffer) {
//         //     Ok(v) => v,
//         //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//         // };
//         // println!("result: {}", s);
        

//         let length = buffer.len();
//         s += length;
//         if length == 0 {
//             break;
//         }

//         // _ = write_file("out.txt", &buffer);
//         reader.consume(length);
//     }

//     let dur = t1.elapsed().as_secs_f64();

//     println!("DUR: {dur}");
//     println!("S = {s}");
    

//     Ok(())
// }




fn write_file(file_name: &str, str_buf: &[u8]) -> std::io::Result<()> {
    let file = OpenOptions::new().append(true).create(true).open(file_name).expect("Eroro");

    let mut buffer = BufWriter::new(file);

    buffer.write_all(str_buf)?;
    buffer.flush()?;

    Ok(())
}