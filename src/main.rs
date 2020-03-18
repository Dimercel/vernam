extern crate pbr;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, BufWriter};
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::process;
use pbr::ProgressBar;


const BUFFER_SIZE: usize = 4096;


// Зашифровывает содержимое файла с помощью файла-ключа используя алгоритм одноразового блокнота
fn cipher_process(source_path: &String, key_path: &String) -> Result<(), io::Error> {
    let source_size = fs::metadata(source_path).unwrap().len();

    let mut source     = File::open(source_path)?;
    let mut cipher     = File::create(format!("{}.vernam", source_path))?;
    let mut key_source = File::open(key_path)?;

    let mut buffer     = [0u8; BUFFER_SIZE];
    let mut key_buf    = [0u8; BUFFER_SIZE];

    let mut pb = ProgressBar::new(source_size / BUFFER_SIZE as u64);
    pb.show_speed = false;
    pb.format("╢▌▌░╟");
    while let Ok(read_count) = source.read(&mut buffer) {
        if read_count == 0 { break; }

        key_source.read(&mut key_buf)?;

        for inx in 0..read_count {
            key_buf[inx] = buffer[inx] ^ key_buf[inx];
        }

        cipher.write(&key_buf[0..read_count])?;
        pb.inc();
    }

    pb.finish_print("Done");

    return Ok(());
}

// Безопасно удаляет файл путем заполнения его нулями и последующим удалением из файловой системы
fn erase_file(path: &String) -> Result<(), io::Error> {
    let file_size = fs::metadata(path).unwrap().len();

    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(1024*1024, file);
    let     buffer = [0u8; BUFFER_SIZE];

    for _ in 0..(file_size / BUFFER_SIZE as u64) {
        writer.write(&buffer)?;
    }

    writer.write(&buffer[0..((file_size % BUFFER_SIZE as u64) as usize)])?;

    fs::remove_file(path)?;

    return Ok(());
}

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

    println!("Starting...");
    match cipher_process(&source_path, &key_path) {
        Err(_) => {
            println!("Error during cipher process!");
            process::exit(0x0001);
        },
        _ => (),
    }

    println!("Safe deletion of the original file. Please, wait...");
    match erase_file(&source_path) {
        Err(_) => {
            println!("Error during safe deletion process!");
            process::exit(0x0001);
        },
        _ => (),
    }

    match fs::rename(format!("{}.vernam", source_path), source_path) {
        Err(_) => {
            println!("Error! Path '{}.vernam' does not exists!", source_path);
            process::exit(0x0001);
        },
        _ => (),
    }

    println!("ok");
}
