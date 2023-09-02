use std::cell::RefCell;
use std::rc::Rc;
use num_derive::FromPrimitive;
use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::chunk::OpCode::OpConstant;
use sanscript_common::debug::disassemble_chunk;
use sanscript_common::value::Value;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::ScannerRef;
use crate::token::TokenType;
use strum::{EnumCount, IntoEnumIterator};

#[repr(usize)]
#[derive(Copy, Clone, FromPrimitive)]
enum Precedence {
    None,
    Assignment,
    // =
    Or,
    // or
    And,
    // and
    Equality,
    // == !=
    Comparison,
    // > < >= <=
    Term,
    // + -
    Factor,
    // * /
    Unary,
    // ! -
    Call,
    // . ()
    Primary,
}

type ParseFn<'a> = fn(&mut Compiler<'a>);

#[derive(Copy, Clone)]
struct ParseRule<'a> {
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}

pub struct Compiler<'a> {
    parser: Parser,
    compiling_chunk: Option<&'a mut Chunk>,
    rules: Vec<ParseRule<'a>>,
    scanner: ScannerRef<'a>,
    source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Compiler<'a> {
        let mut compiler = Compiler {
            parser: Parser::new(),
            compiling_chunk: None,
            rules: vec![ParseRule{infix: None, prefix: None, precedence: Precedence::None};TokenType::COUNT + 1],
            scanner: Rc::new(RefCell::new(Scanner::new(source))),
            source,
        };

        let mut token_index: usize = TokenType::LeftParen.into();
        compiler.rules[token_index] = ParseRule {
            infix: Some(Compiler::grouping),
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::RightParen.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::LeftBrace.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::RightBrace.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Comma.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Dot.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Minus.into();
        compiler.rules[token_index] = ParseRule {
            infix: Some(Compiler::unary),
            prefix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        token_index = TokenType::Plus.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        token_index = TokenType::Semicolon.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Slash.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        token_index = TokenType::Star.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        token_index = TokenType::Bang.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::BangEqual.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Equal.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::EqualEqual.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Greater.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::GreaterEqual.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Less.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::LessEqual.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Identifier.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::String.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Number.into();
        compiler.rules[token_index] = ParseRule {
            infix: Some(Compiler::number),
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::And.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Else.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::False.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::For.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Fn.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::If.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Key.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Match.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Loop.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Nil.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Or.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Print.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Return.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::True.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::Let.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::While.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = 39; //Error
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        token_index = TokenType::EOF.into();
        compiler.rules[token_index] = ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        };

        compiler
    }

    pub fn compile(&mut self, chunk: &'a mut Chunk) -> bool {
        self.compiling_chunk = Some(chunk);

        self.parser.advance(self.scanner.clone());
        self.expression();
        self.parser.consume(TokenType::EOF, "Expect end of expression.".to_string(), self.scanner.clone());

        self.end_compiler();
        return !self.parser.had_error;
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance(self.scanner.clone());
        let previous_token: usize = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).token_type.clone().into();
        let prefix_rule = self.rules[previous_token].prefix;

        if let Some(prefix) = prefix_rule {
            prefix(self);
        } else {
            self.parser.error(String::from("Expect expression."), self.source);
            return;
        }

        let mut current_token_type = self.parser.current.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).token_type.clone();
        let mut current_token_index: usize = current_token_type.clone().into();
        let mut current_token_precedence = self.rules.get(current_token_index).unwrap_or_else(|| { panic!("No rule for token type: {}", current_token_type.clone()) }).precedence;
        while precedence as usize <= current_token_precedence as usize {
            self.parser.advance(self.scanner.clone());
            let previous_token: usize = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).token_type.clone().into();
            let infix_rule = self.rules[previous_token].infix;
            if let Some(infix) = infix_rule {
                infix(self);
            } else {
                break;
            }

            current_token_type = self.parser.current.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).token_type.clone();
            current_token_index = current_token_type.clone().into();
            current_token_precedence = self.rules.get(current_token_index).unwrap_or_else(|| { panic!("No rule for token type: {}", current_token_type.clone()) }).precedence;
        }
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();

        if !self.parser.had_error{
            disassemble_chunk(self.compiling_chunk.as_ref().unwrap_or_else(||{panic!("No chunk present!")}), "code");
        }
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

    pub fn number(&mut self) {
        let value: Value = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") })
            .get_token_string(self.source).parse::<Value>().unwrap_or_else(|_| { panic!("Could not parse token value to number!") });
        self.emit_constant(value);
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.parser.consume(TokenType::RightParen, String::from("Expect ')' after expression"), self.scanner.clone());
    }

    pub fn unary(&mut self) {
        let operator_type = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).token_type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => return
        }
    }

    pub fn binary(&mut self) {
        let operator_type = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).token_type.clone();
        let token_index: usize = operator_type.clone().into();
        let rule = self.rules.get(token_index).unwrap_or_else(|| { panic!("No rule for token type: {}", operator_type.clone()) });
        let next_precedence: Option<Precedence> = num::FromPrimitive::from_usize((rule.precedence as usize) + 1);
        self.parse_precedence(next_precedence.unwrap());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            _ => return
        }
    }
}