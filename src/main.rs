extern crate pbr;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use pbr::ProgressBar;


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

    let source_size = fs::metadata(source_path).unwrap().len();
    if  source_size > fs::metadata(key_path).unwrap().len() {
        println!("The source file must be larger then key-file!");
        process::exit(0x0001);
    }

    let mut source     = File::open(source_path).unwrap();
    let mut cipher     = File::create(format!("{}.vernam", source_path)).unwrap();
    let mut key_source = File::open(key_path).unwrap();

    let mut buffer     = [0u8; BUFFER_SIZE];
    let mut key_buf    = [0u8; BUFFER_SIZE];

    let mut pb = ProgressBar::new(source_size / BUFFER_SIZE as u64);
    pb.format("╢▌▌░╟");
    while let Ok(read_count) = source.read(&mut buffer) {
        if read_count == 0 { break; }

        key_source.read(&mut key_buf);

        for inx in 0..read_count {
            key_buf[inx] = buffer[inx] ^ key_buf[inx];
        }

        cipher.write(&key_buf[0..read_count]);
        pb.inc();
    }

    pb.finish_print("done");
}
