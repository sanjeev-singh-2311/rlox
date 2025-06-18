#![allow(unused_variables)]
#![allow(dead_code)]

use std::{env, fs, path::Path, process::exit};

pub fn run_file(path: String) -> Result<(), u8> {
    let file = fs::read(Path::new(&path));
    match file {
        Ok(content) => {
            let program = match String::from_utf8(content) {
                Ok(s) => s,
                Err(_) => return Err(1),
            };
            // TODO -> make the parser run for the program string
            // run(program);
            println!("{program}");
            Ok(())
        }
        Err(e) => Err(1),
    }
}

pub fn main() {
    let executable_relative_path = env::args().next();
    let argv: Vec<String> = env::args().skip(1).collect();
    let argc = argv.len();

    if argc > 1 {
        println!("usage:: rlox [script] {argc} {argv:?}");
        exit(64);
    }
    if argc == 1 {
        match run_file(argv[0].to_owned()) {
            Err(n) => {
                panic!("Some error occured")
            }
            _ => (),
        }
    } else {
        println!("Run a prompt here")
        // run_prompt()
    }
}
