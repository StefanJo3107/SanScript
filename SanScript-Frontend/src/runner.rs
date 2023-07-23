use std::io;
use std::io::{Read, Write};
use std::fs::File;

pub fn repl() -> io::Result<()>{
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        _ = stdin.read_line(&mut input);
    }
}

pub fn run_file(path: &str) -> io::Result<()>{
    read_file(path)?;
    Ok(())
}

fn read_file(path: &str) -> io::Result<Vec<u8>>{
    let mut source_file = File::open(path)?;
    let mut source = vec![];
    source_file.read_to_end(&mut source)?;

    Ok(source)
}