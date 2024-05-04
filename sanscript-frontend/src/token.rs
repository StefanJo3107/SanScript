use strum_macros::{EnumCount as EnumCountMacro, EnumIter, Display};
use strum::IntoEnumIterator;

#[derive(PartialEq, Clone, Display, Debug, EnumCountMacro, EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    //single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Pipe,
    Semicolon,
    Slash,
    Star,

    //one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //literals
    Identifier,
    String,
    Number,
    HidKey,

    //keywords
    And,
    Else,
    False,
    For,
    Fn,
    If,
    Key,
    Loop,
    Match,
    Nil,
    Or,
    Print,
    Return,
    True,
    Let,
    While,

    //misc
    Error(String),
    EOF,
}

impl Into<usize> for TokenType {
    fn into(self) -> usize {
        let mut value = 1;
        for token in TokenType::iter() {
            if self == token {
                return value
            }
            value += 1;
        }

        return 0
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start_index: usize,
    pub length: usize,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start_index: usize, length: usize, line: usize) -> Token {
        Token {
            token_type,
            start_index,
            length,
            line,
        }
    }

    pub fn get_token_string(&self, source: &str) -> String {
        source[self.start_index..self.start_index + self.length].to_string()
    }
}