#[repr(usize)]
#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    //single character tokens
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,

    //one or two character tokens
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    //literals
    TokenIdentifier,
    TokenString,
    TokenNumber,

    //keywords
    TokenAnd,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFn,
    TokenIf,
    TokenKey,
    TokenLoop,
    TokenMatch,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenTrue,
    TokenLet,
    TokenWhile,

    //misc
    TokenError,
    TokenEOF,
}

pub struct Token{
    pub token_type: TokenType,
    pub start_index: usize,
    pub length: usize,
    pub line: usize,
}

impl<'a> Token{
    pub fn new(token_type: TokenType, start_index: usize, length: usize, line: usize) -> Token {
        Token{
            token_type,
            start_index,
            length,
            line,
        }
    }

    pub fn get_token_string(&self, source: &'a str) -> &'a str {
        &source[self.start_index..self.start_index+self.length]
    }
}