use std::env::args;

use errors::*;

mod errors;

pub struct FoundError {
    domain: String,
    message: String,
}

fn main() {
    let error_code = match args().nth(1) {
        Some(code) => code,
        None => {
            println!("Usage: {} <error code>", args().next().unwrap());
            std::process::exit(1);
        }
    };

    let parsed_error_code = if error_code.contains("0x") {
        error_code.replace("0x", "")
    } else {
        let parsed_code = match error_code.parse::<i64>() {
            Ok(code) => code,
            Err(_) => {
                println!("Error: {error_code} is not a valid error code");
                std::process::exit(1);
            }
        };

        format!("{:X}", parsed_code)
    };

    let errors = find_errors(&parsed_error_code);

    if errors.is_empty() {
        println!("No errors for found code: 0x{parsed_error_code}");
        std::process::exit(1);
    }

    println!("0x{}: ", parsed_error_code);
    for error in errors {
        println!("\t{:<25} |\t{}", error.domain, error.message);
    }
}

pub fn find_errors(hex_code: &str) -> Vec<FoundError> {
    let mut found_errors: Vec<FoundError> = Vec::new();

    if let Some(e) = ANCHOR_ERROR.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Anchor Program".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = METADATA_ERROR.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Token Metadata".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = AUCTION_HOUSE_ERROR.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Auction House".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = AUCTIONEER_ERROR.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Auctioneer".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = CANDY_ERROR.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Candy Machine".to_string(),
            message: e.to_string(),
        });
    }

    found_errors
}
