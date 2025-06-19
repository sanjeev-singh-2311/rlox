#![allow(unused_variables)]
#![allow(dead_code)]

mod error;
mod token;
mod token_type;
mod utils;

use std::{env, process::exit};

pub fn main() {
    let mut arg_iter = env::args();
    let executable_relative_path = match arg_iter.next() {
        Some(s) => s,
        None => "IDK something changed so the file name is not here".to_string(),
    };
    let argv: Vec<String> = arg_iter.collect();
    let argc = argv.len();

    if argc > 1 {
        println!("usage:: rlox [script] {argc} {argv:?}");
        exit(64);
    }
    if argc == 1 {
        match utils::run_file(argv[0].to_owned()) {
            Err(e) => {
                println!("{e:?}")
            }
            _ => (),
        }
    } else {
        match utils::run_prompt() {
            Err(e) => {
                println!("{e:?}")
            }
            _ => (),
        }
    }
}
