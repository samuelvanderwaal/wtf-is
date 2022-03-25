use std::env::args;

mod errors;

pub struct FoundError {
    domain: String,
    message: String,
}

fn main() {
    let hex_code = args().nth(1);

    match hex_code {
        Some(code) => {
            let code = code.to_uppercase();
            let errors = find_errors(&code);

            if errors.is_empty() {
                println!("No errors for found code: {}", code);
                std::process::exit(0);
            }
            println!("0x{}: ", code);
            for error in errors {
                println!("\t{:<25} |\t{}", error.domain, error.message);
            }
        }
        None => {
            println!("No hex code provided!");
            std::process::exit(1);
        }
    }
}

pub fn find_errors(hex_code: &str) -> Vec<FoundError> {
    let mut found_errors: Vec<FoundError> = Vec::new();

    if let Some(e) = errors::ANCHOR_PROGRAM.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Anchor Program".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = errors::AUCTION_HOUSE.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Auction House".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = errors::NFT_CANDY_MACHINE.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "NFT Candy Machine V1".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = errors::NFT_CANDY_MACHINE_V2.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "NFT Candy Machine V2".to_string(),
            message: e.to_string(),
        });
    }

    if let Some(e) = errors::TOKEN_METADATA.get(hex_code).cloned() {
        found_errors.push(FoundError {
            domain: "Token Metadata".to_string(),
            message: e.to_string(),
        });
    }

    found_errors
}
