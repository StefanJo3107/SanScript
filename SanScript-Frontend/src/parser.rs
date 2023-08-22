use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

type ScannerRef<'a> = Rc<RefCell<Scanner<'a>>>;

pub struct Parser {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: ScannerRef) {
        self.previous = self.current.clone();

        loop {
            let mut scanner = scanner.borrow_mut();
            let token = scanner.scan_token();
            self.current = Some(token);

            match self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).token_type {
                TokenType::Error(_) => {}
                _ => break
            }

            self.error_at_current(self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).get_token_string(scanner.source), scanner.source);
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String, scanner: ScannerRef) {
        if self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).token_type == token_type {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message, scanner.borrow().source);
    }

    fn error_at_current(&mut self, message: String, source: &str) {
        self.error_at(self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None.") }), message, source);
        self.panic_mode = true;
        self.had_error = true;
    }

    fn error(&mut self, message: String, source: &str) {
        self.error_at(self.previous.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None.") }), message, source);
        self.panic_mode = true;
        self.had_error = true;
    }

    fn error_at(&self, token: &Token, message: String, source: &str) {
        if self.panic_mode { return; }
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else if matches!(token.token_type, TokenType::Error(_)) {
            //Nothing
        } else {
            eprint!(" at {}", token.get_token_string(source));
        }

        eprintln!(": {}", message);
    }
}