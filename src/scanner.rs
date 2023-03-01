use super::error_handling::ErrorReporter;

pub struct Scanner<'a> {
    error_reporter: &'a mut ErrorReporter,

    src: &'a str,
    pub tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(error_reporter: &'a mut ErrorReporter, src: &'a str) -> Scanner<'a> {
        let mut scanner = Scanner {
            error_reporter,

            src,
            tokens: [].to_vec(),

            start: 0,
            current: 0,
            line: 0,
        };
        scanner.scan_tokens();

        scanner
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_tokens(&mut self) -> &[Token] {
        if self.tokens.is_empty() {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }

            self.tokens.push(Token {
                token_type: TokenType::Eof,
                lexeme: "",
                literal: TokenLiteral::None,
                line: self.line,
            });
        }

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, TokenLiteral::None),
            ')' => self.add_token(TokenType::RightParen, TokenLiteral::None),
            '{' => self.add_token(TokenType::LeftBrace, TokenLiteral::None),
            '}' => self.add_token(TokenType::RightBrace, TokenLiteral::None),
            ',' => self.add_token(TokenType::Comma, TokenLiteral::None),
            '.' => self.add_token(TokenType::Period, TokenLiteral::None),
            '-' => self.add_token(TokenType::Minus, TokenLiteral::None),
            '+' => self.add_token(TokenType::Plus, TokenLiteral::None),
            ';' => self.add_token(TokenType::Semicolon, TokenLiteral::None),
            '*' => self.add_token(TokenType::Asterisk, TokenLiteral::None),
            default => self.error_reporter.error(
                self.line,
                format!("Unexpected character '{}'.", default).as_ref(),
            ),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: TokenLiteral<'a>) {
        let text = &self.src[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let next_char = self.src.chars().collect::<Vec<char>>()[self.current];
        self.current += 1;

        next_char
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
    None,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,

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
