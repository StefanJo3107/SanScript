use std::env;
use std::process::exit;

pub mod runner;

fn main() {
    if env::args().len() == 1 {
        if let Err(e) = runner::repl() {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    } else if env::args().len() == 2 {
        if let Err(e) = runner::run_file(env::args().last().unwrap().as_str()) {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    } else {
        eprintln!("Usage: SanScript [path]");
        exit(1);
    }
}
