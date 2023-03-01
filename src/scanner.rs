pub struct Scanner {
    pub src: String,
}

impl Scanner {
    pub fn scan_tokens(&self) -> Vec<Token> {
        self.src
            .split_ascii_whitespace()
            .map(|token_str| Token {
                raw: token_str.to_string(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub raw: String,
}
