use std::collections::HashMap;

use super::error_handling::ErrorReporter;

pub struct Scanner<'a> {
    error_reporter: &'a mut ErrorReporter,
    keywords: HashMap<&'a str, TokenType>,

    src: &'a str,
    tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(error_reporter: &'a mut ErrorReporter, src: &'a str) -> Scanner<'a> {
        let mut scanner = Scanner {
            error_reporter,
            keywords: HashMap::new(),

            src,
            tokens: [].to_vec(),

            start: 0,
            current: 0,
            line: 0,
        };

        scanner.populate_keywords();
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
                value: TokenValue::None,
                line: self.line,
            });
        }

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Skip whitespace, but increment line counter when a newline char is encountered.
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            // Single char token matching
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Period, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Asterisk, None),

            // Single or double char token matching
            '!' => {
                let token_type = if self.char_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.char_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None)
            }
            '<' => {
                let token_type = if self.char_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None)
            }
            '>' => {
                let token_type = if self.char_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None)
            }

            // Either matches a division token, or will consume a comment.
            '/' => {
                if self.char_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }

            // Generates a token for a string.
            '"' => self.string(),

            // If any other character is found, try to parse either a number or identifier/keyword from source,
            // otherwise simply inform the user of the unexpected character.
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.error_reporter
                        .error(self.line, format!("Unexpected character '{}'.", c).as_ref())
                }
            }
        }
    }

    fn add_token<T1>(&mut self, token_type: TokenType, value: T1)
    where
        T1: Into<Option<TokenValue<'a>>>,
    {
        let text = &self.src[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            value: value.into().unwrap_or(TokenValue::None),
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let next_char = self.src.chars().nth(self.current).unwrap();
        self.current += 1;

        next_char
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.src.chars().nth(self.current).unwrap() != expected) {
            false
        } else {
            self.current += 1;

            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.src.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.src.len() {
            return '\0';
        }

        self.src.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_reporter.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value = &self.src[(self.start + 1)..(self.current - 1)];
        self.add_token(TokenType::String, TokenValue::String(value))
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let value = &self.src[self.start..self.current];

        self.add_token(
            TokenType::Number,
            TokenValue::Number(value.parse().unwrap()),
        );
    }

    // Parse either an identifier or a reserved keyword.
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let value = &self.src[self.start..self.current];
        let token_type = self
            .keywords
            .get(value)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(token_type, None);
    }

    fn populate_keywords(&mut self) {
        self.keywords.insert("and", TokenType::And);
        self.keywords.insert("class", TokenType::Class);
        self.keywords.insert("else", TokenType::Else);
        self.keywords.insert("false", TokenType::False);
        self.keywords.insert("for", TokenType::For);
        self.keywords.insert("fn", TokenType::Fn);
        self.keywords.insert("if", TokenType::If);
        self.keywords.insert("let", TokenType::Let);
        self.keywords.insert("null", TokenType::Null);
        self.keywords.insert("or", TokenType::Or);
        self.keywords.insert("print", TokenType::Print);
        self.keywords.insert("return", TokenType::Return);
        self.keywords.insert("super", TokenType::Super);
        self.keywords.insert("this", TokenType::This);
        self.keywords.insert("true", TokenType::True);
        self.keywords.insert("while", TokenType::While);
    }

    pub fn display_tokens(&self) {
        for token in &self.tokens {
            println!(
                "{}",
                match token.value {
                    TokenValue::String(val) => match token.token_type {
                        TokenType::String => format!("    String: \"{}\"", val),
                        _ => format!("   Keyword: {}", val),
                    },
                    TokenValue::Number(val) => format!("    Number: {}", val),
                    TokenValue::None => match token.token_type {
                        TokenType::Identifier => format!("Identifier: {}", token.lexeme),
                        _ => format!(
                            "{}: {}",
                            if self.keywords.get(token.lexeme).is_some() {
                                "   Keyword"
                            } else {
                                "     Other"
                            },
                            token.lexeme
                        ),
                    },
                }
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub value: TokenValue<'a>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue<'a> {
    String(&'a str),
    Number(f64),
    None,
}

#[derive(Debug, Clone, PartialEq)]
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

    // Types
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Let,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    Eof,
}
