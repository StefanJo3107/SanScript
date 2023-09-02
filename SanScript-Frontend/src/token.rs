use std::fmt;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(PartialEq, Clone, Debug, EnumCountMacro, EnumIter)]
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
impl Into<usize> for TokenType{
    fn into(self) -> usize {
        match self
        {
            TokenType::LeftParen => 1,
            TokenType::RightParen => 2,
            TokenType::LeftBrace => 3,
            TokenType::RightBrace => 4,
            TokenType::Comma => 5,
            TokenType::Dot => 6,
            TokenType::Plus => 7,
            TokenType::Minus => 8,
            TokenType::Semicolon => 9,
            TokenType::Slash => 10,
            TokenType::Star => 11,

            TokenType::Bang => 12,
            TokenType::BangEqual => 13,
            TokenType::Equal => 14,
            TokenType::EqualEqual => 15,
            TokenType::Greater => 16,
            TokenType::GreaterEqual => 17,
            TokenType::Less => 18,
            TokenType::LessEqual => 19,

            TokenType::Identifier => 20,
            TokenType::String => 21,
            TokenType::Number => 22,

            TokenType::And => 23,
            TokenType::Else => 24,
            TokenType::False => 25,
            TokenType::For => 26,
            TokenType::Fn => 27,
            TokenType::If => 28,
            TokenType::Key => 29,
            TokenType::Loop => 30,
            TokenType::Match => 31,
            TokenType::Nil => 32,
            TokenType::Or => 33,
            TokenType::Print => 34,
            TokenType::Return => 35,
            TokenType::True => 36,
            TokenType::Let => 37,
            TokenType::While => 38,

            TokenType::Error(_) => 39,
            TokenType::EOF => 40,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),

            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),

            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),

            TokenType::And => write!(f, "AND"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::For => write!(f, "FOR"),
            TokenType::Fn => write!(f, "FN"),
            TokenType::If => write!(f, "IF"),
            TokenType::Key => write!(f, "KEY"),
            TokenType::Loop => write!(f, "LOOP"),
            TokenType::Match => write!(f, "MATCH"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Let => write!(f, "LET"),
            TokenType::While => write!(f, "WHILE"),

            TokenType::Error(_) => write!(f, "ERROR"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub start_index: usize,
    pub length: usize,
    pub line: usize,
}

impl Token{
    pub fn new(token_type: TokenType, start_index: usize, length: usize, line: usize) -> Token {
        Token{
            token_type,
            start_index,
            length,
            line,
        }
    }

    pub fn get_token_string(&self, source: &str) -> String {
        source[self.start_index..self.start_index+self.length].to_string()
    }
}