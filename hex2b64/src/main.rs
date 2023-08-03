use hex::FromHex;
use hex::ToHex;
use base64::{encode};

fn main() {
    
    println!("Enter a hexadecimal string:");
    let mut input_hex = String::new();
    std::io::stdin().read_line(&mut input_hex).expect("Failed to read input");

   
    let input_hex = input_hex.trim();

    
    let bytes = match Vec::<u8>::from_hex(input_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    
    let base64_encoded = encode(&bytes);

    println!("Base64 encoded: {}", base64_encoded);
}
