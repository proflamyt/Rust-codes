use std::io::{Read, Write};
use std::fs::File;
use std::io::{self, BufRead};
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};


fn main() {
    let file_content = read_file("encrypted.txt");  
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = "\0x";

    let block_bytes = split_blocks(decode(file_content));

    let initial = xor_vectors(decrypt_blocks(block_bytes[0], key), iv);

    for i in (1..block_bytes-1) {

        xor_vectors(decrypt_blocks(block_bytes[i], key), block_bytes[i-1]);

    }

    let ciphertext_base64 = encode(ciphertext);

    println!("Base64-encoded Ciphertext: {}", ciphertext_base64);

}


fn encrypt_block(key, plain_text) -> String {

    let cipher = Aes128::ecb_encryptor(&key);
    let mut ciphertext = Vec::new();
    cipher.encrypt(&mut ciphertext, &plain_text).expect("encryption failed");
    ciphertext

}


fn read_file(file: &str) -> String{
    let file = File::open().unwrap();
    let mut base64_file = String::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_content: String = line.unwrap();
        base64_file += &line_content;
    }
   
    base64_file
    
}

fn decrypt_blocks(block1 : &[u8], key : &[u8]) {

    let mut decryptor = aes::ecb_decryptor(KeySize::KeySize128, key, NoPadding);

}

fn split_blocks(blocks : &[u8]) -> Vec<&[u8]>{

    let len: usize =  blocks.len();
    // if LEN % 16 != 0 {
    //     return None;
    // }
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
