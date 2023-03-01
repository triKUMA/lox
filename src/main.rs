use std::env;

mod lox;

fn main() -> color_eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;

    let mut lox = lox::Lox::new();

    match config.script_path {
        Some(script_path) => lox.run_file(&script_path),
        None => lox.run_prompt(),
    }
}

#[derive(Debug)]
struct Config {
    script_path: Option<String>,
}

fn parse_config(args: &[String]) -> Result<Config, color_eyre::Report> {
    if args.len() == 1 {
        let script_path = None;

        return Ok(Config { script_path });
    } else if args.len() == 2 {
        let script_path = Some(args[1].clone());

        return Ok(Config { script_path });
    }

    println!("Usage: lox [script_path]");
    std::process::exit(64);
}
