use std::env::args;

mod errors;

fn main() {
    let hex_code = args().skip(1).next();

    match hex_code {
        Some(code) => {
            let code = code.to_uppercase();
            let message = parse(&code);
            println!("0x{}: {}", code, message);
        }
        None => {
            println!("No hex code provided!");
            std::process::exit(1);
        }
    }
}

pub fn parse(hex_code: &str) -> &str {
    match errors::ERRORS.get(hex_code).cloned() {
        Some(e) => return e,
        None => {
            println!("No match found for that code!");
            std::process::exit(1);
        }
    }
}
