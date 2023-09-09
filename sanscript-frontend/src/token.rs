use strum_macros::{EnumCount as EnumCountMacro, EnumIter, Display};
use strum::IntoEnumIterator;

#[derive(PartialEq, Clone, Display, Debug, EnumCountMacro, EnumIter)]
pub enum TokenType {
    //single character tokens
    #[strum(serialize = "LEFT_PAREN")]
    LeftParen,
    #[strum(serialize = "RIGHT_PAREN")]
    RightParen,
    #[strum(serialize = "LEFT_BRACE")]
    LeftBrace,
    #[strum(serialize = "RIGHT_BRACE")]
    RightBrace,
    #[strum(serialize = "COMMA")]
    Comma,
    #[strum(serialize = "DOT")]
    Dot,
    #[strum(serialize = "MINUS")]
    Minus,
    #[strum(serialize = "PLUS")]
    Plus,
    #[strum(serialize = "SEMICOLON")]
    Semicolon,
    #[strum(serialize = "SLASH")]
    Slash,
    #[strum(serialize = "STAR")]
    Star,

    //one or two character tokens
    #[strum(serialize = "BANG")]
    Bang,
    #[strum(serialize = "BANG_EQUAL")]
    BangEqual,
    #[strum(serialize = "EQUAL")]
    Equal,
    #[strum(serialize = "EQUAL_EQUAL")]
    EqualEqual,
    #[strum(serialize = "GREATER")]
    Greater,
    #[strum(serialize = "GREATER_EQUAL")]
    GreaterEqual,
    #[strum(serialize = "LESS")]
    Less,
    #[strum(serialize = "LESS_EQUAL")]
    LessEqual,

    //literals
    #[strum(serialize = "IDENTIFIER")]
    Identifier,
    #[strum(serialize = "STRING")]
    String,
    #[strum(serialize = "NUMBER")]
    Number,

    //keywords
    #[strum(serialize = "AND")]
    And,
    #[strum(serialize = "ELSE")]
    Else,
    #[strum(serialize = "FALSE")]
    False,
    #[strum(serialize = "FOR")]
    For,
    #[strum(serialize = "FN")]
    Fn,
    #[strum(serialize = "IF")]
    If,
    #[strum(serialize = "KEY")]
    Key,
    #[strum(serialize = "LOOP")]
    Loop,
    #[strum(serialize = "MATCH")]
    Match,
    #[strum(serialize = "NIL")]
    Nil,
    #[strum(serialize = "OR")]
    Or,
    #[strum(serialize = "PRINT")]
    Print,
    #[strum(serialize = "RETURN")]
    Return,
    #[strum(serialize = "TRUE")]
    True,
    #[strum(serialize = "LET")]
    Let,
    #[strum(serialize = "WHILE")]
    While,

    //misc
    #[strum(serialize = "ERROR")]
    Error(String),
    #[strum(serialize = "EOF")]
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

#[derive(Clone)]
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