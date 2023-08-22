use std::cell::RefCell;
use std::rc::Rc;
use sanscript_common::chunk::Chunk;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;

pub fn compile(source: String, chunk: &mut Chunk) -> bool {
    let scanner = Rc::new(RefCell::new(Scanner::new(source.as_str())));
    let mut parser = Parser::new();

    parser.advance(scanner.clone());
    expression();
    parser.consume(TokenType::EOF, "Expect end of expression.".to_string(), scanner.clone());

    return !parser.had_error;
}


fn expression() {}