use std::cell::RefCell;
use std::rc::Rc;
use num_derive::FromPrimitive;
use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::chunk::OpCode::OpConstant;
use sanscript_common::value::{Number, Value};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::ScannerRef;
use crate::token::{Token, TokenType};
use strum::EnumCount;

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

type ParseFn<'a> = fn(&mut Compiler<'a>, bool);

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
            rules: vec![ParseRule { infix: None, prefix: None, precedence: Precedence::None }; TokenType::COUNT + 1],
            scanner: Rc::new(RefCell::new(Scanner::new(source))),
            source,
        };

        macro_rules! add_table_entry {
            ($token_type: expr, Some($prefix: expr), Some($infix: expr), $precedence: expr) => {
                let token_index: usize = $token_type.into();
                compiler.rules[token_index] = ParseRule {
                    prefix: Some($prefix),
                    infix: Some($infix),
                    precedence: $precedence,
                };
            };
            ($token_type: expr, Some($prefix: expr), None, $precedence: expr) => {
                let token_index: usize = $token_type.into();
                compiler.rules[token_index] = ParseRule {
                    prefix: Some($prefix),
                    infix: None,
                    precedence: $precedence,
                };
            };
            ($token_type: expr, None, Some($infix: expr), $precedence: expr) => {
                let token_index: usize = $token_type.into();
                compiler.rules[token_index] = ParseRule {
                    prefix: None,
                    infix: Some($infix),
                    precedence: $precedence,
                };
            };
            ($token_type: expr, None, None, $precedence: expr) => {
                let token_index: usize = $token_type.into();
                compiler.rules[token_index] = ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: $precedence,
                };
            };
        }

        add_table_entry!(TokenType::LeftParen, Some(Compiler::grouping), None, Precedence::None);
        add_table_entry!(TokenType::RightParen, None, None, Precedence::None);
        add_table_entry!(TokenType::LeftBrace, None, None, Precedence::None);
        add_table_entry!(TokenType::RightBrace, None, None, Precedence::None);
        add_table_entry!(TokenType::Comma, None, None, Precedence::None);
        add_table_entry!(TokenType::Dot, None, None, Precedence::None);
        add_table_entry!(TokenType::Minus, Some(Compiler::unary), Some(Compiler::binary), Precedence::Term);
        add_table_entry!(TokenType::Plus, None, Some(Compiler::binary), Precedence::Term);
        add_table_entry!(TokenType::Semicolon, None, None, Precedence::None);
        add_table_entry!(TokenType::Slash, None, Some(Compiler::binary), Precedence::Factor);
        add_table_entry!(TokenType::Star, None, Some(Compiler::binary), Precedence::Factor);
        add_table_entry!(TokenType::Bang, Some(Compiler::unary), None, Precedence::None);
        add_table_entry!(TokenType::BangEqual, None, Some(Compiler::binary), Precedence::Equality);
        add_table_entry!(TokenType::Equal, None, None, Precedence::None);
        add_table_entry!(TokenType::EqualEqual, None, Some(Compiler::binary), Precedence::Equality);
        add_table_entry!(TokenType::Greater, None, Some(Compiler::binary), Precedence::Comparison);
        add_table_entry!(TokenType::GreaterEqual, None, Some(Compiler::binary), Precedence::Comparison);
        add_table_entry!(TokenType::Less, None, Some(Compiler::binary), Precedence::Comparison);
        add_table_entry!(TokenType::LessEqual, None, Some(Compiler::binary), Precedence::Comparison);
        add_table_entry!(TokenType::Identifier, Some(Compiler::variable), None, Precedence::None);
        add_table_entry!(TokenType::String, Some(Compiler::string), None, Precedence::None);
        add_table_entry!(TokenType::Number, Some(Compiler::number), None, Precedence::None);
        add_table_entry!(TokenType::And, None, None, Precedence::None);
        add_table_entry!(TokenType::Else, None, None, Precedence::None);
        add_table_entry!(TokenType::False, Some(Compiler::literal), None, Precedence::None);
        add_table_entry!(TokenType::For, None, None, Precedence::None);
        add_table_entry!(TokenType::Fn, None, None, Precedence::None);
        add_table_entry!(TokenType::If, None, None, Precedence::None);
        add_table_entry!(TokenType::Key, None, None, Precedence::None);
        add_table_entry!(TokenType::Match, None, None, Precedence::None);
        add_table_entry!(TokenType::Loop, None, None, Precedence::None);
        add_table_entry!(TokenType::Nil, Some(Compiler::literal), None, Precedence::None);
        add_table_entry!(TokenType::Or, None, None, Precedence::None);
        add_table_entry!(TokenType::Print, None, None, Precedence::None);
        add_table_entry!(TokenType::Return, None, None, Precedence::None);
        add_table_entry!(TokenType::True, Some(Compiler::literal), None, Precedence::None);
        add_table_entry!(TokenType::Let, None, None, Precedence::None);
        add_table_entry!(TokenType::While, None, None, Precedence::None);
        let error_token = TokenType::Error("".to_string());
        add_table_entry!(error_token, None, None, Precedence::None);
        add_table_entry!(TokenType::EOF, None, None, Precedence::None);

        compiler
    }

    pub fn compile(&mut self, chunk: &'a mut Chunk) -> bool {
        self.compiling_chunk = Some(chunk);

        self.parser.advance(self.scanner.clone());

        while !self.match_token(TokenType::EOF) {
            self.declaration();
        }

        self.end_compiler();
        return !self.parser.had_error;
    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Let) {
            self.variable_declaration();
        } else {
            self.statement();
        }
    }

    fn variable_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name");

        if self.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil);
        }

        self.parser.consume(TokenType::Semicolon, String::from("Expect ';' after value"), self.scanner.clone());
        self.define_variable(global);
    }

    fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.parser.consume(TokenType::Identifier, String::from(error_msg), self.scanner.clone());
        let identifier = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") });
        return self.identifier_constant(identifier.clone());
    }

    fn identifier_constant(&mut self, identifier: Token) -> usize {
        let token_string = identifier.get_token_string(self.source);
        let chunk = self.compiling_chunk.as_mut().unwrap_or_else(|| { panic!("Current chunk is not set!") });
        let ident_value = Value::ValString(token_string);
        let offset = chunk.has_constant(&ident_value);
        if offset == -1 {
            return chunk.add_constant(ident_value);
        }
        return offset as usize;
    }

    fn define_variable(&mut self, global: usize) {
        self.emit_byte(OpCode::OpDefineGlobal(global));
    }

    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check_token(token_type) {
            return false;
        }
        self.parser.advance(self.scanner.clone());
        true
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        let current_token = self.parser.current.as_ref().unwrap_or_else(|| { panic!("Parser does not have current token processed!") });
        let current_type = current_token.token_type.clone();
        return current_type == token_type;
    }

    fn print_statement(&mut self) {
        self.expression();
        self.parser.consume(TokenType::Semicolon, String::from("Expect ';' after value"), self.scanner.clone());
        self.emit_byte(OpCode::OpPrint);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.parser.consume(TokenType::Semicolon, String::from("Expect ';' after value"), self.scanner.clone());
        self.emit_byte(OpCode::OpPop);
    }
    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance(self.scanner.clone());
        let previous_token: usize = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).token_type.clone().into();
        let prefix_rule = self.rules[previous_token].prefix;

        let can_assign = precedence as usize <= Precedence::Assignment as usize;
        if let Some(prefix) = prefix_rule {
            prefix(self, can_assign);
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
                infix(self, can_assign);
            } else {
                break;
            }

            current_token_type = self.parser.current.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).token_type.clone();
            current_token_index = current_token_type.clone().into();
            current_token_precedence = self.rules.get(current_token_index).unwrap_or_else(|| { panic!("No rule for token type: {}", current_token_type.clone()) }).precedence;
        }

        if can_assign && self.match_token(TokenType::Equal){
            self.parser.error(String::from("Invalid assignment target"), self.source);
        }
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn emit_byte(&mut self, byte: OpCode) {
        self.compiling_chunk.as_mut()
            .unwrap_or_else(|| { panic!("Current chunk is not set!") })
            .write_chunk(byte, self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).line);
    }

    pub fn emit_bytes(&mut self, bytes: &[OpCode]) {
        for byte in bytes {
            self.compiling_chunk.as_mut()
                .unwrap_or_else(|| { panic!("Current chunk is not set!") })
                .write_chunk(*byte, self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).line);
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let chunk = self.compiling_chunk.as_mut().unwrap_or_else(|| { panic!("Current chunk is not set!") });
        let mut offset = chunk.has_constant(&value);
        if offset == -1 {
            offset = chunk.add_constant(value) as isize;
        }

        self.emit_byte(OpConstant(offset as usize));
    }

    pub fn number(&mut self, _can_assign: bool) {
        let value: Number = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") })
            .get_token_string(self.source).parse::<Number>().unwrap_or_else(|_| { panic!("Could not parse token value to number!") });
        self.emit_constant(Value::ValNumber(value));
    }

    pub fn literal(&mut self, _can_assign: bool) {
        let token_type = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).token_type.clone();

        match token_type {
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            _ => return
        }
    }

    pub fn string(&mut self, _can_assign: bool) {
        let value = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") })
            .get_token_string(self.source);
        let string_literal = &value[1..value.len() - 1].to_string();
        self.emit_constant(Value::ValString(string_literal.to_owned()));
    }

    pub fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.parser.consume(TokenType::RightParen, String::from("Expect ')' after expression"), self.scanner.clone());
    }

    pub fn unary(&mut self, _can_assign: bool) {
        let operator_type = self.parser.previous.as_ref().unwrap_or_else(|| { panic!("Parser does not have processed token!") }).token_type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            _ => return
        }
    }

    pub fn binary(&mut self, _can_assign: bool) {
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
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::BangEqual => self.emit_bytes(&[OpCode::OpEqual, OpCode::OpNot]),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater),
            TokenType::Less => self.emit_byte(OpCode::OpLess),
            TokenType::GreaterEqual => self.emit_bytes(&[OpCode::OpLess, OpCode::OpNot]),
            TokenType::LessEqual => self.emit_bytes(&[OpCode::OpGreater, OpCode::OpNot]),
            _ => return
        }
    }

    pub fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous.as_ref().unwrap_or_else(|| { panic!("No token has been processed!") }).clone(), can_assign);
    }

    pub fn named_variable(&mut self, identifier: Token, can_assign: bool) {
        let arg = self.identifier_constant(identifier);
        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_byte(OpCode::OpSetGlobal(arg));
        } else {
            self.emit_byte(OpCode::OpGetGlobal(arg));
        }
    }
}