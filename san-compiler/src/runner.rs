use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::exit;
use postcard::to_allocvec;
use sanscript_common::value::FunctionType;
use crate::compiler::Compiler;

pub fn run() {
    if env::args().len() == 3 {
        let args: Vec<String> = env::args().collect();
        if let Err(e) = run_file(args[1].as_str(), args[2].as_str()) {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    } else {
        eprintln!("Usage: SanScript [source path] [destination path]");
        exit(1);
    }
}

pub fn run_file(source_path: &str, dest_path: &str) -> io::Result<()> {
    read_file(source_path, dest_path);
    Ok(())
}

fn read_file(source_path: &str, dest_path: &str) {
    let mut source_file = File::open(source_path).unwrap_or_else(|e|{panic!("Error opening file at path {}: {}", source_path, e.to_string())});
    let mut source: String = String::from("");
    source_file.read_to_string(&mut source).unwrap_or_else(|e|{panic!("Error reading file content: {}", e.to_string())});
    println!("{}", source);
    let mut compiler = Compiler::new(source.as_str(), FunctionType::Script);
    if let Some(function) = compiler.compile() {
        let output = to_allocvec(&function).unwrap_or_else(|e|{panic!("Error serializing compiler result: {}", e)});
        let mut file = File::create(dest_path).unwrap_or_else(|e|{panic!("Error opening file at path {}: {}", dest_path, e.to_string())});
        file.write_all(output.as_slice()).expect("Error writing serialized data to a file");
    }
}