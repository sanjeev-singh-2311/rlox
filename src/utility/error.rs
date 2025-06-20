use std::process::exit;

pub fn show_error(line: u32, message: String) {
    report(line, "", message)
}

fn report(line: u32, location: &str, message: String) {
    eprintln!("[line \"{line}\"] Error {location} : {message}");
    exit(64)
}
