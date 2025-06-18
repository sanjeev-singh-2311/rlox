use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

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

pub fn run_prompt() {
    let std_input = io::stdin();
    let mut reader = io::BufReader::new(std_input);

    let mut program = String::new();
    print!("> ");
    while let Ok(chars) = reader.read_line(&mut program) {
        if chars == 0 {
            break;
        }
        // TODO -> make the parser run for the program string
        // run(program);
        println!("The program -> {program}");
        // Clear the string to take input again
        program.clear();
        print!("> ")
    }
}
