#[path = "./error_handling.rs"]
pub mod error_handling;
#[path = "./scanner.rs"]
pub mod scanner;
#[path = "./syntax_tree.rs"]
pub mod syntax_tree;

use self::syntax_tree::Expr;
use self::{error_handling::ErrorReporter, scanner::Scanner};
use std::{
    fs,
    io::{self, Write},
};

pub struct Lox {
    pub error_reporter: ErrorReporter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            error_reporter: ErrorReporter::new(),
        }
    }

    fn run(&mut self, src: &str) -> Result<(), color_eyre::Report> {
        let scanner = Scanner::new(&mut self.error_reporter, src);

        let syntax_tree = Expr::Binary(
            Box::new(Expr::Unary(
                scanner::Token {
                    token_type: scanner::TokenType::Minus,
                    lexeme: "-",
                    value: scanner::TokenValue::None,
                    line: 0,
                },
                Box::new(Expr::Value(scanner::TokenValue::Number(123.0))),
            )),
            scanner::Token {
                token_type: scanner::TokenType::Asterisk,
                lexeme: "*",
                value: scanner::TokenValue::None,
                line: 0,
            },
            Box::new(Expr::Grouping(Box::new(Expr::Value(
                scanner::TokenValue::Number(45.67),
            )))),
        );

        println!("{syntax_tree}");

        Ok(())
    }

    pub fn run_file(&mut self, script_path: &str) -> Result<(), color_eyre::Report> {
        let src = fs::read_to_string(script_path)?;

        self.run(&src)?;

        if self.error_reporter.had_error {
            std::process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), color_eyre::Report> {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.trim().to_string();

            match input.as_ref() {
                "exit" | "end" | "quit" => break,
                _ => self.run(&input)?,
            }

            self.error_reporter.had_error = false;
        }

        Ok(())
    }
}
