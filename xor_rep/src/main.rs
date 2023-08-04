use hex::{ToHex};
use std::io::{self, BufRead};

fn main() {
    

     let mut password = String::new();

    let stdin = io::stdin();

    let mut first_input = stdin.lock().lines();

    let mut all_bytes: Vec<u8> = Vec::new();

    println!("Enter lines of text (press Enter without typing to finish):");

    while let Some(Ok(line)) = first_input.next() {
        if line.is_empty() {
            break; 
        }
        all_bytes.extend_from_slice(line.as_bytes());
    }
    drop(first_input);
    println!("Enter password:");

    std::io::stdin().read_line(&mut password).expect("Enter Corect password"); 


    let password = password.trim();

    let pass_hex = password.as_bytes().to_vec();

    let s = xor_vectors(all_bytes, pass_hex);

    let hex_string = s.encode_hex::<String>();

    println!("Result Hex encoded: {}", hex_string);

}


fn xor_vectors(s: Vec<u8>, p: Vec<u8>)  -> Vec<u8> {
    
    

    let mut result = Vec::with_capacity(s.len());

    for (index, character) in s.iter().enumerate() {
        
        result.push(character ^ p[index % p.len()]);
    }


    return result;

}