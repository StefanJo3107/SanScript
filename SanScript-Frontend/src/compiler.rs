use crate::scanner::Scanner;
use crate::token::TokenType;

pub fn compile(source: String) {
    let mut scanner = Scanner::new(&source);

    let mut line: isize = -1;

    println!("\x1B[4mLINE | TYPE ID | TOKEN\x1B[0m");

    loop {
        let token = scanner.scan_token();
        if token.line as isize != line {
            print!("{:<5}  ", token.line);
            line = token.line as isize;
        } else {
            print!("|      ");
        }

        println!("{:<9} '{}'", token.token_type as usize, token.get_token_string());

        if token.token_type == TokenType::EOF {
            break;
        }
    }
}