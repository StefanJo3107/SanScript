use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    //start index of the current lexeme
    start_index: usize,
    //current char index of the current lexeme
    current_index: usize,
    source: &'a str,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            start_index: 0,
            current_index: 0,
            source,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start_index = self.current_index;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c: char = self.advance();

        if Scanner::is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            '.' => return self.make_token(TokenType::Dot),
            ',' => return self.make_token(TokenType::Comma),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::BangEqual);
                }
                return self.make_token(TokenType::Bang);
            }
            '=' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::EqualEqual);
                }
                return self.make_token(TokenType::Equal);
            }
            '>' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::GreaterEqual);
                }
                return self.make_token(TokenType::Greater);
            }
            '<' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::LessEqual);
                }
                return self.make_token(TokenType::Less);
            }
            '"' => {
                return string();
            }
            _ => ()
        }

        return self.error_token("Unexpected character.");
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\t' | '\r' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current_index >= self.source.len()
    }

    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start_index, self.current_index - self.start_index, self.source, self.line)
    }

    pub fn error_token(&self, message: &'a str) -> Token {
        Token::new(TokenType::Error, 0, message.len(), message, self.line)
    }

    pub fn number(&mut self) -> Token {
        while Scanner::is_digit(self.peek()){
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()){
            self.advance();
            while Scanner::is_digit(self.peek()){
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    pub fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() { return self.error_token("Unterminated string."); }

        self.advance();
        return self.make_token(TokenType::String);
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current_index).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") })
    }

    pub fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source.chars().nth(self.current_index + 1).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") })
    }

    pub fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source.chars().nth(self.current_index - 1).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") })
    }

    pub fn match_next(&mut self, next: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current_index).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") }) != next {
            return false;
        }

        self.current_index += 1;
        return true;
    }
}