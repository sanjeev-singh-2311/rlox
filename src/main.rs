#![allow(unused_variables)]
#![allow(dead_code)]

mod scanner;
mod tokens;
mod utility;

use std::{env, process::exit};
use utility::error as rlox_error;
use utility::utils;

pub fn main() {
    let mut arg_iter = env::args();

    /*
        WARN: I literally have no idea what do in case some platform does not give exectable path as
        first arguement but still passes the other args, I'll actually be cooked if that were to happen
        Might actually have to compare the `executable_relative_path` variable to the executable
        path or something AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHHHHHHHHHH!!
    */

    let executable_relative_path = match arg_iter.next() {
        Some(s) => s,
        None => {
            rlox_error::show_error(
                0,
                "not only are the args missing, the executable path itself is missing lol"
                    .to_owned(),
            );
            exit(64)
        }
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
