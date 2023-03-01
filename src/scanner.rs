pub struct Scanner<'a> {
    pub src: &'a str,
}

impl Scanner<'_> {
    pub fn scan_tokens(&self) -> Vec<Token> {
        self.src
            .lines()
            .enumerate()
            .flat_map(|(index, line)| {
                line.split_ascii_whitespace().map(move |token_str| Token {
                    token_type: TokenType::Null,
                    lexeme: token_str,
                    literal: TokenLiteral::String(token_str),
                    line: index + 1,
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: TokenLiteral<'a>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum TokenLiteral<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
