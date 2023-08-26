use std::cell::RefCell;
use std::rc::Rc;
use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::chunk::OpCode::OpConstant;
use sanscript_common::value::Value;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;

pub struct Compiler<'a> {
    parser: Parser,
    compiling_chunk: Option<&'a mut Chunk>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        Compiler {
            parser: Parser::new(),
            compiling_chunk: None,
        }
    }

    pub fn compile(&mut self, source: String, chunk: &'a mut Chunk) -> bool {
        let scanner = Rc::new(RefCell::new(Scanner::new(source.as_str())));
        self.compiling_chunk = Some(chunk);

        self.parser.advance(scanner.clone());
        self.expression();
        self.parser.consume(TokenType::EOF, "Expect end of expression.".to_string(), scanner.clone());

        self.end_compiler();
        return !self.parser.had_error;
    }

    pub fn expression(&mut self) {}


    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn emit_byte(&mut self, byte: OpCode) {
        self.compiling_chunk.as_mut()
            .unwrap_or_else(|| { panic!("Current chunk is not set!") })
            .write_chunk(byte, self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).line);
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let offset = self.compiling_chunk.as_mut().unwrap_or_else(|| { panic!("Current chunk is not set!") }).add_constant(value);
        self.emit_byte(OpConstant(offset));
    }

    pub fn number(&mut self, source: &str) {
        let value: Value = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") })
            .get_token_string(source).parse::<Value>().unwrap_or_else(|_| { panic!("Could not parse token value to number!") });
        self.emit_constant(value);
    }
}