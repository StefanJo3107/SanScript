#[repr(usize)]
#[derive(PartialEq, Copy, Clone)]
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
    Error,
    EOF,
}

#[derive(Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub start_index: usize,
    pub length: usize,
    pub source: String,
    pub line: usize,
}

impl Token{
    pub fn new(token_type: TokenType, start_index: usize, length: usize, source: String, line: usize) -> Token {
        Token{
            token_type,
            start_index,
            length,
            source,
            line,
        }
    }

    pub fn get_token_string(&self) -> String {
        self.source[self.start_index..self.start_index+self.length].to_string()
    }
}