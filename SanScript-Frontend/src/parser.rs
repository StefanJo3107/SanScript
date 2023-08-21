use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

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

    pub fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            let scanned_token = scanner.scan_token();
            self.current = Some(scanned_token);

            if self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).get_token_string());
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String, scanner: &mut Scanner){
        if self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None. This is really weird!") }).token_type == token_type{
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(self.current.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None.") }), message);
        self.panic_mode = true;
        self.had_error = true;
    }

    fn error(&mut self, message: String) {
        self.error_at(self.previous.as_ref().unwrap_or_else(|| { panic!("Parsed token is of type None.") }), message);
        self.panic_mode = true;
        self.had_error = true;
    }

    fn error_at(&self, token: &Token, message: String) {
        if self.panic_mode { return; }
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            //Nothing
        } else {
            eprint!(" at {}", token.get_token_string());
        }

        eprintln!(": {}", message);
    }
}