use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::path::Path;
use std::process;


const BUFFER_SIZE: usize = 4096;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Parameters not specified!");
        process::exit(0x0001);
    }

    let source_path = &args[1];
    let key_path    = &args[2];

    if !(Path::new(source_path).exists()) ||
       !(Path::new(key_path).exists()) {

        println!("The specified file does not exist!");
        process::exit(0x0001);
    }

    if fs::metadata(source_path).unwrap().len() > fs::metadata(key_path).unwrap().len() {
        println!("The source file must be larger then key-file!");
        process::exit(0x0001);
    }

    let mut source     = File::open(source_path).unwrap();
    let mut cipher     = File::create(format!("{}.vernam", source_path)).unwrap();
    let mut key_source = File::open(key_path).unwrap();

    let mut buffer     = [0u8; BUFFER_SIZE];
    let mut key_buf    = [0u8; BUFFER_SIZE];
    let mut cipher_buf = [0u8; BUFFER_SIZE];

    while let Ok(read_count) = source.read(&mut buffer) {
        if read_count == 0 { break; }

        key_source.read(&mut key_buf);

        for inx in 0..read_count {
            cipher_buf[inx] = buffer[inx] ^ key_buf[inx];
        }

        cipher.write(&cipher_buf[0..read_count]);
    }
}
