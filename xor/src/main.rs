use hex::{FromHex, ToHex};

fn main() {
    println!("Hello, world!");

    let mut hex_input = String::new();
    let mut shex_input = String::new();

    std::io::stdin().read_line(&mut hex_input).expect("Enter Coreect hex format"); 

    std::io::stdin().read_line(&mut shex_input).expect("Enter Coreect hex format"); 


    let input_hex = hex_input.trim();
    let sinput_hex = shex_input.trim();

    let hex = Vec::<u8>::from_hex(input_hex).unwrap();

    let shex = Vec::<u8>::from_hex(sinput_hex).unwrap();

    let  c = xor_vectors(&hex, &shex);

    match c {
        Some(bytes) => {
            // Convert the Vec<u8> to a hexadecimal string
            let hex_string = bytes.encode_hex::<String>();
            println!("Result Hex encoded: {}", hex_string);
        }
        None => println!("Error: Vectors must have equal lengths to compute XOR."),
    }
}




fn xor_vectors(a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    
    if a.len() != b.len() {
        return None; // XOR operation requires equal-length vectors
    }

    let mut result = Vec::with_capacity(a.len());

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }

    Some(result)
}





