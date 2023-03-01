#[path = "./scanner.rs"]
mod scanner;

use std::{
    fs,
    io::{self, Write},
};

use crate::lox::scanner::Scanner;

pub fn run_file(script_path: &str) -> Result<(), color_eyre::Report> {
    let src = fs::read_to_string(script_path)?;

    run(&src)
}

pub fn run_prompt() -> Result<(), color_eyre::Report> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        match input.as_ref() {
            "exit" | "end" | "quit" => break,
            _ => run(&input)?,
        }
    }

    Ok(())
}

fn run(src: &str) -> Result<(), color_eyre::Report> {
    println!("\n{src}\n");

    let scanner = Scanner {
        src: src.to_string(),
    };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token.raw);
    }

    Ok(())
}
