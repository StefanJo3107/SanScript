use sanscript_common::keycodes::HID_KEY_STRINGS;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    //start index of the current lexeme
    start_index: usize,
    //current char index of the current lexeme
    current_index: usize,
    pub source: &'a str,
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

    pub fn tokenize_source(&mut self) {
        let mut line: isize = -1;

        println!("\x1B[4mLINE |   TOKEN TYPE   | TOKEN  \x1B[0m");

        loop {
            let token = self.scan_token();
            if token.line as isize != line {
                print!("{:<5}  ", token.line);
                line = token.line as isize;
            } else {
                print!("|      ");
            }

            println!("{:16} '{}'", token.token_type.to_string(), token.get_token_string(self.source));

            if token.token_type == TokenType::EOF {
                break;
            }
        }

        println!();
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start_index = self.current_index;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c: char = self.advance();

        if Scanner::is_capital(c) {
            //TODO
            return self.hid();
        }

        if Scanner::is_alpha(c) {
            return self.identifier();
        }

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
            '|' => return self.make_token(TokenType::Pipe),
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
                return self.string();
            }
            _ => ()
        }

        return self.error_token("Unexpected character.");
    }

    pub fn skip_whitespace(&mut self) {
        if self.is_at_end() {
            return;
        }

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

    pub fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    pub fn is_capital(c: char) -> bool {
        return c >= 'A' && c <= 'Z';
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start_index, self.current_index - self.start_index, self.line)
    }

    pub fn error_token(&self, message: &'a str) -> Token {
        Token::new(TokenType::Error(message.to_string()), 0, message.len(), self.line)
    }

    pub fn hid(&mut self) -> Token {
        while Scanner::is_capital(self.peek()) || Scanner::is_digit(self.peek()) || self.peek().eq(&'_') {
            self.advance();
        }
        self.make_token(self.hid_type())
    }

    pub fn identifier(&mut self) -> Token {
        while Scanner::is_alpha(self.peek()) || Scanner::is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    pub fn hid_type(&self) -> TokenType {
        for hid_key in HID_KEY_STRINGS {
            if self.current_index - self.start_index == hid_key.len() && &self.source[self.start_index..self.current_index] == hid_key {
                return TokenType::HidKey;
            }
        }

        TokenType::Identifier
    }

    pub fn identifier_type(&self) -> TokenType {
        let start_char = self.source.chars().nth(self.start_index).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") });
        return match start_char {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'l' => self.check_keyword(1, 2, "et", TokenType::Let),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            't' => self.check_keyword(1, 3, "rue", TokenType::True),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current_index - self.start_index > 1 {
                    let second_char = self.source.chars().nth(self.start_index + 1).unwrap_or_else(|| { panic!("Tried to index source code outside of its bounds!") });
                    return match second_char {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'n' => {
                            if self.current_index - self.start_index == 2 {
                                return TokenType::Fn;
                            }

                            TokenType::Identifier
                        }
                        _ => TokenType::Identifier
                    };
                }

                TokenType::Identifier
            }
            _ => TokenType::Identifier
        };
    }

    pub fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current_index - self.start_index == start + length && &self.source[self.start_index + start..self.current_index] == rest {
            return token_type;
        }

        return TokenType::Identifier;
    }

    pub fn number(&mut self) -> Token {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
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
        if self.is_at_end() {
            return '\0';
        }
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