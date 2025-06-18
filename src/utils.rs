use std::{fs, path::Path};

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
