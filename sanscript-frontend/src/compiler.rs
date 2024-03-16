use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::ScannerRef;
use num_derive::FromPrimitive;
use sanscript_common::chunk::OpCode::OpConstant;
use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::value::{FunctionData, FunctionType, Number, Value};
use std::cell::RefCell;
use std::isize;
use std::rc::Rc;
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

#[derive(Debug)]
pub struct Local {
    token: Token,
    depth: isize,
}

pub struct Compiler<'a> {
    parser: Parser,
    function: FunctionData,
    function_type: FunctionType,
    rules: Vec<ParseRule<'a>>,
    scanner: ScannerRef<'a>,
    source: &'a str,
    locals: Vec<Local>,
    scope_depth: isize,
}

impl<'a> Compiler<'a> {
    pub fn token_table_init(compiler: &mut Compiler) {
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

        add_table_entry!(
            TokenType::LeftParen,
            Some(Compiler::grouping),
            Some(Compiler::call),
            Precedence::Call
        );
        add_table_entry!(TokenType::RightParen, None, None, Precedence::None);
        add_table_entry!(TokenType::LeftBrace, None, None, Precedence::None);
        add_table_entry!(TokenType::RightBrace, None, None, Precedence::None);
        add_table_entry!(TokenType::Comma, None, None, Precedence::None);
        add_table_entry!(TokenType::Dot, None, None, Precedence::None);
        add_table_entry!(
            TokenType::Minus,
            Some(Compiler::unary),
            Some(Compiler::binary),
            Precedence::Term
        );
        add_table_entry!(
            TokenType::Plus,
            None,
            Some(Compiler::binary),
            Precedence::Term
        );
        add_table_entry!(TokenType::Semicolon, None, None, Precedence::None);
        add_table_entry!(
            TokenType::Slash,
            None,
            Some(Compiler::binary),
            Precedence::Factor
        );
        add_table_entry!(
            TokenType::Star,
            None,
            Some(Compiler::binary),
            Precedence::Factor
        );
        add_table_entry!(
            TokenType::Bang,
            Some(Compiler::unary),
            None,
            Precedence::None
        );
        add_table_entry!(
            TokenType::BangEqual,
            None,
            Some(Compiler::binary),
            Precedence::Equality
        );
        add_table_entry!(TokenType::Equal, None, None, Precedence::None);
        add_table_entry!(
            TokenType::EqualEqual,
            None,
            Some(Compiler::binary),
            Precedence::Equality
        );
        add_table_entry!(
            TokenType::Greater,
            None,
            Some(Compiler::binary),
            Precedence::Comparison
        );
        add_table_entry!(
            TokenType::GreaterEqual,
            None,
            Some(Compiler::binary),
            Precedence::Comparison
        );
        add_table_entry!(
            TokenType::Less,
            None,
            Some(Compiler::binary),
            Precedence::Comparison
        );
        add_table_entry!(
            TokenType::LessEqual,
            None,
            Some(Compiler::binary),
            Precedence::Comparison
        );
        add_table_entry!(
            TokenType::Identifier,
            Some(Compiler::variable),
            None,
            Precedence::None
        );
        add_table_entry!(
            TokenType::String,
            Some(Compiler::string),
            None,
            Precedence::None
        );
        add_table_entry!(
            TokenType::Number,
            Some(Compiler::number),
            None,
            Precedence::None
        );
        add_table_entry!(TokenType::And, None, Some(Compiler::and), Precedence::And);
        add_table_entry!(TokenType::Else, None, None, Precedence::None);
        add_table_entry!(
            TokenType::False,
            Some(Compiler::literal),
            None,
            Precedence::None
        );
        add_table_entry!(TokenType::For, None, None, Precedence::None);
        add_table_entry!(TokenType::Fn, None, None, Precedence::None);
        add_table_entry!(TokenType::If, None, None, Precedence::None);
        add_table_entry!(TokenType::Key, None, None, Precedence::None);
        add_table_entry!(TokenType::Match, None, None, Precedence::None);
        add_table_entry!(TokenType::Loop, None, None, Precedence::None);
        add_table_entry!(
            TokenType::Nil,
            Some(Compiler::literal),
            None,
            Precedence::None
        );
        add_table_entry!(TokenType::Or, None, Some(Compiler::or), Precedence::Or);
        add_table_entry!(TokenType::Print, None, None, Precedence::None);
        add_table_entry!(TokenType::Return, None, None, Precedence::None);
        add_table_entry!(
            TokenType::True,
            Some(Compiler::literal),
            None,
            Precedence::None
        );
        add_table_entry!(TokenType::Let, None, None, Precedence::None);
        add_table_entry!(TokenType::While, None, None, Precedence::None);
        let error_token = TokenType::Error("".to_string());
        add_table_entry!(error_token, None, None, Precedence::None);
        add_table_entry!(TokenType::EOF, None, None, Precedence::None);
    }
    pub fn new(source: &'a str, function_type: FunctionType) -> Compiler<'a> {
        let mut compiler = Compiler {
            parser: Parser::new(),
            function: FunctionData::new(),
            function_type,
            rules: vec![
                ParseRule {
                    infix: None,
                    prefix: None,
                    precedence: Precedence::None,
                };
                TokenType::COUNT + 1
            ],
            scanner: Rc::new(RefCell::new(Scanner::new(source))),
            source,
            locals: vec![],
            scope_depth: 0,
        };

        if function_type != FunctionType::Script {
            compiler.function.name = compiler.parser.previous.as_ref().unwrap().get_token_string(compiler.source);
        }
        compiler.locals.push(Local { depth: 0, token: Token::new(TokenType::Nil, 0, 0, 0) });
        Compiler::token_table_init(&mut compiler);
        compiler
    }

    pub fn new_from_existing(parser: Parser, scanner: ScannerRef<'a>, source: &'a str, function_type: FunctionType) -> Compiler<'a> {
        let mut compiler = Compiler {
            parser,
            function: FunctionData::new(),
            function_type,
            rules: vec![
                ParseRule {
                    infix: None,
                    prefix: None,
                    precedence: Precedence::None,
                };
                TokenType::COUNT + 1
            ],
            scanner,
            source,
            locals: vec![],
            scope_depth: 0,
        };

        if function_type != FunctionType::Script {
            compiler.function.name = compiler.parser.previous.as_ref().unwrap().get_token_string(compiler.source);
        }
        compiler.locals.push(Local { depth: 0, token: Token::new(TokenType::Nil, 0, 0, 0) });
        Compiler::token_table_init(&mut compiler);
        compiler
    }

    fn get_chunk(&self) -> &Chunk {
        &self
            .function.chunk
    }

    fn get_chunk_mut(&mut self) -> &mut Chunk {
        &mut self
            .function.chunk
    }

    pub fn compile(&mut self) -> Option<FunctionData> {
        self.parser.advance(self.scanner.clone());

        while !self.match_token(TokenType::EOF) {
            self.declaration();
        }

        let had_error = self.parser.had_error;
        let function = self.end_compiler();
        if !had_error { Some(function) } else { None }
    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Fn) {
            self.fn_declaration();
        } else if self.match_token(TokenType::Let) {
            self.variable_declaration();
        } else {
            self.statement();
        }
    }

    fn fn_declaration(&mut self) {
        let global = self.parse_variable("Expect function name");
        self.mark_initialized();
        self.function(FunctionType::Function);
        self.define_variable(global);
    }

    fn function(&mut self, function_type: FunctionType) {
        let mut compiler = Compiler::new_from_existing(self.parser.clone(), self.scanner.clone(), self.source.clone(), function_type);

        compiler.begin_scope();
        compiler.parser.consume(TokenType::LeftParen, String::from("Expect '(' after function name"), self.scanner.clone());
        if !compiler.check_token(TokenType::RightParen) {
            loop {
                compiler.function.arity += 1;
                if compiler.function.arity > 255 {
                    compiler.parser.error(String::from("Can't have more tha 255 parameters"), compiler.source);
                }

                let constant = compiler.parse_variable("Expect parameter name");
                compiler.define_variable(constant);

                if !compiler.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        compiler.parser.consume(TokenType::RightParen, String::from("Expect ')' after parameters"), self.scanner.clone());
        compiler.parser.consume(TokenType::LeftBrace, String::from("Expect '{' before function body"), self.scanner.clone());
        compiler.block();

        let function = compiler.end_compiler();
        let offset = self.get_chunk_mut().add_constant(Value::ValFunction(function));
        self.emit_byte(OpConstant(offset));
        self.parser = compiler.parser;
        self.scanner = compiler.scanner;
    }

    fn call(&mut self, _can_assign: bool) {
        let arg_count = self.argument_list();
        self.emit_byte(OpCode::OpCall(arg_count));
    }

    fn argument_list(&mut self) -> usize {
        let mut arg_count = 0;
        if !self.check_token(TokenType::RightParen) {
            loop {
                self.expression();
                arg_count += 1;
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.parser.consume(TokenType::RightParen, String::from("Expect ')' after arguments"), self.scanner.clone());
        arg_count
    }

    fn variable_declaration(&mut self) {
        let var_name = self.parse_variable("Expect variable name");

        if self.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil);
        }

        self.parser.consume(
            TokenType::Semicolon,
            String::from("Expect ';' after value"),
            self.scanner.clone(),
        );
        self.define_variable(var_name);
    }

    fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.parser.consume(
            TokenType::Identifier,
            String::from(error_msg),
            self.scanner.clone(),
        );
        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }

        let identifier = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"));
        return self.identifier_constant(identifier.clone());
    }

    fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }

        let variable = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"))
            .clone();

        for local in &self.locals {
            if local.depth != -1 && local.depth < self.scope_depth {
                break;
            }

            if self.identifiers_equal(&variable, &local.token) {
                self.parser.error(
                    String::from("Variable redeclaration in the same scope"),
                    self.source,
                );
            }
        }

        self.add_local(variable);
    }

    fn define_variable(&mut self, global: usize) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }

        self.emit_byte(OpCode::OpDefineGlobal(global));
    }

    fn identifier_constant(&mut self, identifier: Token) -> usize {
        let token_string = identifier.get_token_string(self.source);
        let chunk = self.get_chunk_mut();
        let ident_value = Value::ValString(token_string);
        let offset = chunk.has_constant(&ident_value);
        if offset == -1 {
            return chunk.add_constant(ident_value);
        }
        return offset as usize;
    }

    fn identifiers_equal(&self, a: &Token, b: &Token) -> bool {
        return a.get_token_string(self.source) == b.get_token_string(self.source);
    }

    fn add_local(&mut self, token: Token) {
        let local: Local = Local { token, depth: -1 };

        self.locals.push(local);
    }

    fn mark_initialized(&mut self) {
        if self.scope_depth == 0 { return; }
        let local = self
            .locals
            .last_mut()
            .unwrap_or_else(|| panic!("Locals array is empty!"));
        local.depth = self.scope_depth;
    }

    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else if self.match_token(TokenType::If) {
            self.if_statement();
        } else if self.match_token(TokenType::While) {
            self.while_statement();
        } else if self.match_token(TokenType::For) {
            self.for_statement();
        } else if self.match_token(TokenType::Return){
          self.return_statement();
        } else {
            self.expression_statement();
        }
    }

    fn if_statement(&mut self) {
        self.parser.consume(
            TokenType::LeftParen,
            String::from("Expect '(' after 'if'"),
            self.scanner.clone(),
        );
        self.expression();
        self.parser.consume(
            TokenType::RightParen,
            String::from("Expect ')' after condition"),
            self.scanner.clone(),
        );

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse(0xff));
        self.emit_byte(OpCode::OpPop);
        self.statement();
        let else_jump = self.emit_jump(OpCode::OpJump(0xff));
        self.patch_jump(then_jump);
        self.emit_byte(OpCode::OpPop);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    fn while_statement(&mut self) {
        let loop_start = self.get_chunk().len();
        self.parser.consume(
            TokenType::LeftParen,
            String::from("Expect '(' after 'while'"),
            self.scanner.clone(),
        );
        self.expression();
        self.parser.consume(
            TokenType::RightParen,
            String::from("Expect ')' after condition"),
            self.scanner.clone(),
        );

        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse(0xff));
        self.emit_byte(OpCode::OpPop);
        self.statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OpPop);
    }

    fn for_statement(&mut self) {
        self.begin_scope();
        self.parser.consume(
            TokenType::LeftParen,
            String::from("Expect '(' after 'for'"),
            self.scanner.clone(),
        );
        if self.match_token(TokenType::Semicolon) {} else if self.match_token(TokenType::Let) {
            self.variable_declaration();
        } else {
            self.expression_statement();
        }

        let mut loop_start = self.get_chunk().len();
        let mut exit_jump = 0;
        if !self.match_token(TokenType::Semicolon) {
            self.expression();
            self.parser.consume(
                TokenType::Semicolon,
                String::from("Expect ';' after loop condition."),
                self.scanner.clone(),
            );
            exit_jump = self.emit_jump(OpCode::OpJumpIfFalse(0xff));
            self.emit_byte(OpCode::OpPop);
        }

        if !self.match_token(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::OpJump(0xff));
            let increment_start = self.get_chunk().len();
            self.expression();
            self.emit_byte(OpCode::OpPop);
            self.parser.consume(
                TokenType::RightParen,
                String::from("Expect ')' after for clauses."),
                self.scanner.clone(),
            );
            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        self.statement();
        self.emit_loop(loop_start);

        if exit_jump != 0 {
            self.patch_jump(exit_jump);
            self.emit_byte(OpCode::OpPop);
        }

        self.end_scope();
    }

    fn return_statement(&mut self) {
        if self.function_type == FunctionType::Script {
            self.parser.error(String::from("Can't return from top-level code"), self.source);
        }

        if self.match_token(TokenType::Semicolon) {
            self.emit_return();
        } else{
            self.expression();
            self.parser.consume(TokenType::Semicolon, String::from("Expect ';' after return value"), self.scanner.clone());
            self.emit_byte(OpCode::OpReturn);
        }
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction);
        self.get_chunk().len() - 1
    }

    fn patch_jump(&mut self, address: usize) {
        let jump = self.get_chunk().len() - address - 1;
        let new_code = match self.get_chunk().get_code(address)
        {
            OpCode::OpJumpIfFalse(value) => Some(OpCode::OpJumpIfFalse(jump)),
            OpCode::OpJumpIfTrue(value) => Some(OpCode::OpJumpIfTrue(jump)),
            OpCode::OpJump(value) => Some(OpCode::OpJump(jump)),
            _ => None,
        };

        if let Some(code) = new_code {
            self.get_chunk_mut().set_code(code, address);
        }
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::OpLoop(
            self.get_chunk().len() - loop_start + 1,
        ));
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        for i in (0..self.locals.len()).rev() {
            if self
                .locals
                .get(i)
                .unwrap_or_else(|| panic!("No local variable with given index"))
                .depth
                >= self.scope_depth + 1
            {
                self.locals.remove(i);
                self.emit_byte(OpCode::OpPop);
            }
        }
    }

    fn block(&mut self) {
        while !self.check_token(TokenType::RightBrace) && !self.check_token(TokenType::EOF) {
            self.declaration();
        }

        self.parser.consume(
            TokenType::RightBrace,
            String::from("Expect '}' after block."),
            self.scanner.clone(),
        );
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check_token(token_type) {
            return false;
        }
        self.parser.advance(self.scanner.clone());
        true
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        let current_token = self
            .parser
            .current
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have current token processed!"));
        let current_type = current_token.token_type.clone();
        return current_type == token_type;
    }

    fn print_statement(&mut self) {
        self.expression();
        self.parser.consume(
            TokenType::Semicolon,
            String::from("Expect ';' after value"),
            self.scanner.clone(),
        );
        self.emit_byte(OpCode::OpPrint);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.parser.consume(
            TokenType::Semicolon,
            String::from("Expect ';' after value"),
            self.scanner.clone(),
        );
        self.emit_byte(OpCode::OpPop);
    }
    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance(self.scanner.clone());
        let previous_token: usize = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"))
            .token_type
            .clone()
            .into();
        let prefix_rule = self.rules[previous_token].prefix;

        let can_assign = precedence as usize <= Precedence::Assignment as usize;
        if let Some(prefix) = prefix_rule {
            prefix(self, can_assign);
        } else {
            self.parser
                .error(String::from("Expect expression."), self.source);
            return;
        }

        let mut current_token_type = self
            .parser
            .current
            .as_ref()
            .unwrap_or_else(|| panic!("No token has been processed!"))
            .token_type
            .clone();
        let mut current_token_index: usize = current_token_type.clone().into();
        let mut current_token_precedence = self
            .rules
            .get(current_token_index)
            .unwrap_or_else(|| panic!("No rule for token type: {}", current_token_type.clone()))
            .precedence;

        while precedence as usize <= current_token_precedence as usize {
            self.parser.advance(self.scanner.clone());
            let previous_token: usize = self
                .parser
                .previous
                .as_ref()
                .unwrap_or_else(|| panic!("Parser does not have processed token!"))
                .token_type
                .clone()
                .into();
            let infix_rule = self.rules[previous_token].infix;
            if let Some(infix) = infix_rule {
                infix(self, can_assign);
            } else {
                break;
            }

            current_token_type = self
                .parser
                .current
                .as_ref()
                .unwrap_or_else(|| panic!("No token has been processed!"))
                .token_type
                .clone();
            current_token_index = current_token_type.clone().into();
            current_token_precedence = self
                .rules
                .get(current_token_index)
                .unwrap_or_else(|| panic!("No rule for token type: {}", current_token_type.clone()))
                .precedence;
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.parser
                .error(String::from("Invalid assignment target"), self.source);
        }
    }

    fn end_compiler(&mut self) -> FunctionData {
        self.emit_return();
        self.function.clone()
    }

    fn emit_byte(&mut self, byte: OpCode) {
        let parser_line = self.parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!")).line;
        self.get_chunk_mut().write_chunk(
            byte,
            parser_line,
        );
    }

    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        for byte in bytes {
            let parser_line = self.parser
                .previous
                .as_ref()
                .unwrap_or_else(|| panic!("Parser does not have processed token!")).line;
            self.get_chunk_mut().write_chunk(
                *byte,
                parser_line,
            );
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpNil);
        self.emit_byte(OpCode::OpReturn);
    }

    fn emit_constant(&mut self, value: Value) {
        let chunk = self.get_chunk_mut();
        let mut offset = chunk.has_constant(&value);
        if offset == -1 {
            offset = chunk.add_constant(value) as isize;
        }

        self.emit_byte(OpConstant(offset as usize));
    }

    fn number(&mut self, _can_assign: bool) {
        let value: Number = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"))
            .get_token_string(self.source)
            .parse::<Number>()
            .unwrap_or_else(|_| panic!("Could not parse token value to number!"));
        self.emit_constant(Value::ValNumber(value));
    }

    fn literal(&mut self, _can_assign: bool) {
        let token_type = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("No token has been processed!"))
            .token_type
            .clone();

        match token_type {
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            _ => return,
        }
    }

    fn string(&mut self, _can_assign: bool) {
        let value = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"))
            .get_token_string(self.source);
        let string_literal = &value[1..value.len() - 1].to_string();
        self.emit_constant(Value::ValString(string_literal.to_owned()));
    }

    fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.parser.consume(
            TokenType::RightParen,
            String::from("Expect ')' after expression"),
            self.scanner.clone(),
        );
    }

    fn unary(&mut self, _can_assign: bool) {
        let operator_type = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("Parser does not have processed token!"))
            .token_type
            .clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            _ => return,
        }
    }

    fn binary(&mut self, _can_assign: bool) {
        let operator_type = self
            .parser
            .previous
            .as_ref()
            .unwrap_or_else(|| panic!("No token has been processed!"))
            .token_type
            .clone();
        let token_index: usize = operator_type.clone().into();
        let rule = self
            .rules
            .get(token_index)
            .unwrap_or_else(|| panic!("No rule for token type: {}", operator_type.clone()));
        let next_precedence: Option<Precedence> =
            num::FromPrimitive::from_usize((rule.precedence as usize) + 1);
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
            _ => return,
        }
    }

    fn and(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse(0xff));

        self.emit_byte(OpCode::OpPop);
        self.parse_precedence(Precedence::And);

        self.patch_jump(end_jump);
    }

    fn or(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfTrue(0xff));

        self.emit_byte(OpCode::OpPop);
        self.parse_precedence(Precedence::Or);

        self.patch_jump(end_jump);
    }

    fn variable(&mut self, can_assign: bool) {
        self.named_variable(
            self.parser
                .previous
                .as_ref()
                .unwrap_or_else(|| panic!("No token has been processed!"))
                .clone(),
            can_assign,
        );
    }

    fn named_variable(&mut self, identifier: Token, can_assign: bool) {
        let get_op: OpCode;
        let set_op: OpCode;
        let arg = self.resolve_local(&identifier);

        if arg != -1 {
            get_op = OpCode::OpGetLocal(arg as usize);
            set_op = OpCode::OpSetLocal(arg as usize);
        } else {
            let arg = self.identifier_constant(identifier);
            get_op = OpCode::OpGetGlobal(arg);
            set_op = OpCode::OpSetGlobal(arg);
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_byte(set_op);
        } else {
            self.emit_byte(get_op);
        }
    }

    fn resolve_local(&mut self, identifier: &Token) -> isize {
        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];
            if self.identifiers_equal(&local.token, &identifier) {
                if local.depth == -1 {
                    self.parser.error(
                        String::from("Can't read local variable in its own initializer"),
                        self.source,
                    );
                }
                return i as isize;
            }
        }

        return -1;
    }
}
