use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::exit;

pub fn run() {
    if env::args().len() == 1 {
        if let Err(e) = repl() {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    } else if env::args().len() == 2 {
        if let Err(e) = run_file(env::args().last().unwrap().as_str()) {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    } else {
        eprintln!("Usage: SanScript [path]");
        exit(1);
    }
}

pub fn repl() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        _ = stdin.read_line(&mut input);
    }
}

pub fn run_file(path: &str) -> io::Result<()> {
    read_file(path)?;
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut source_file = File::open(path)?;
    let mut source: String = String::from("");
    source_file.read_to_string(&mut source)?;

    Ok(source)
}
