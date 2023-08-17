use crate::token::Token;
use crate::token::TokenType::TokenEOF;

pub struct Scanner<'a>{
    //start index of the current lexeme
    start_index: usize,
    //current char index of the current lexeme
    current_index: usize,
    source: &'a String,
    line: usize
}

impl<'a> Scanner<'a>{
    pub fn new(source: &'a String) -> Scanner{
        Scanner{
            start_index: 0,
            current_index: 0,
            source,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        Token::new(TokenEOF, 0, 0, 0)
    }
}