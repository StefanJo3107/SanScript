use crate::scanner::Scanner;
use crate::token::TokenType;

pub fn compile(source: String){
    let mut scanner = Scanner::new(&source);

    let mut line: isize = -1;

    loop{
        let token = scanner.scan_token();
        if token.line as isize != line{
            print!("{:>4} ", token.line);
            line = token.line as isize;
        }else{
            print!("   | ");
        }

        println!("{:>2} '{}'", token.token_type as usize, token.get_token_string());

        if token.token_type == TokenType::TokenEOF{
            break;
        }
    }
}