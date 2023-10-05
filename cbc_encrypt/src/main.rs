use std::fs::File;
use std::io::{self, BufRead};
use base64::{Engine as _, engine::general_purpose};
use aes::Aes128;
use aes::cipher::{
    BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use hex::FromHex;

fn main() {
    let file_content = read_file("encrypted.txt");  
    let key_str = "YELLOW SUBMARINE".as_bytes();

    let iv_string = "00000000000000000000000000000000";

    let mut result: Vec<u8> = Vec::new();

    let iv = Vec::<u8>::from_hex(iv_string).expect("Unable to hexify");

    println!("{:?}", iv);

    let key= GenericArray::clone_from_slice(key_str);
    

    let decoded = &general_purpose::STANDARD_NO_PAD.decode(file_content).unwrap();

    let block_bytes = split_blocks(decoded);

    let initial_plain = xor_vectors(&decrypt_blocks(block_bytes[0], key), &iv).expect("cant xor");

    
    // 

    result.extend(initial_plain.iter());

    for i in 1..block_bytes.len() {
        
        let  text = xor_vectors(&decrypt_blocks(block_bytes[i], key), block_bytes[i-1]).expect("cant xor");

        result.extend(text);
    }

    // let ciphertext_base64 = encode(ciphertext);
    let first = String::from_utf8(result).expect("can't ");

    println!("Result: {}", first);

}



fn read_file(file: &str) -> String {
    let file = File::open(file).unwrap();
    let mut base64_file = String::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_content: String = line.unwrap();
        base64_file += &line_content;
    }
   
    base64_file   
}

fn decrypt_blocks(block1 : &[u8], key: GenericArray<u8, aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UTerm, aes::cipher::typenum::B1>, aes::cipher::typenum::B0>, aes::cipher::typenum::B0>, aes::cipher::typenum::B0>, aes::cipher::typenum::B0>> )-> Vec<u8> {

    let cipher = Aes128::new(&key);

    let mut block = GenericArray::clone_from_slice(block1);

    cipher.decrypt_block(&mut block);

    block.into_iter().collect::<Vec<u8>>()
}

fn split_blocks(blocks : &[u8]) -> Vec<&[u8]>{

    let len: usize =  blocks.len();
   
    let mut block_bytes = Vec::with_capacity(len/16);

    for i in (0..len).step_by(16) {
        block_bytes.push(&blocks[i..i+16]);
    }

    block_bytes
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
