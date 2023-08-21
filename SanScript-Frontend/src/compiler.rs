use sanscript_common::chunk::Chunk;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;

pub fn compile(source: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();

    parser.advance(&mut scanner);
    expression();
    parser.consume(TokenType::EOF, "Expect end of expression.".to_string(), &mut scanner);

    return !parser.had_error;
}


fn expression() {}