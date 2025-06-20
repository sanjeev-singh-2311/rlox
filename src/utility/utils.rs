use super::error::show_error;
use crate::scanner::Scanner;
use crate::tokens::token::Token;
use std::io::{BufRead, Write};
use std::{fs, io, path::Path};

pub fn run_file(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read(Path::new(&path))?;
    let program = String::from_utf8(file)?;
    // TODO -> make the parser run for the program string
    // run(program);
    println!("{program}");
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let std_input = io::stdin();
    let mut reader = io::BufReader::new(std_input);

    loop {
        let mut program = String::new();
        print!("> ");
        io::stdout().flush()?;
        let chars = reader.read_line(&mut program)?;
        if chars == 0 {
            break;
        }
        run(program);
    }
    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    scanner.scan_all_tokens();
    let tokens: Vec<Token> = scanner.tokens;
    for token in tokens.into_iter() {
        println!("{token:?}");
    }
}
