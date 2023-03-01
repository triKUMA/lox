#[path = "./scanner.rs"]
mod scanner;

#[path = "./error_handling.rs"]
pub mod error_handling;

use std::{
    fs,
    io::{self, Write},
};

use crate::lox::{error_handling::ErrorReporter, scanner::Scanner};

pub struct Lox {
    pub error_reporter: ErrorReporter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            error_reporter: ErrorReporter::new(),
        }
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

    fn run(&mut self, src: &str) -> Result<(), color_eyre::Report> {
        let lines_max_len = src.lines().map(|line| line.len()).max().unwrap_or(1);

        println!(
            "\n{}\n{}\n{}\n",
            "-".repeat(lines_max_len),
            src.trim(),
            "-".repeat(lines_max_len)
        );

        let scanner = Scanner::new(&mut self.error_reporter, src);

        scanner.display_tokens();

        Ok(())
    }
}
