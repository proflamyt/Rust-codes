extern crate crypto;
extern crate base64;

use std::io::{Read, Write};
use crypto::aes::{self, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor};
use base64::{Engine as _, engine::general_purpose};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    
    // let mut file = File::open("encrypted.txt").expect("Failed to open file");
    let my_str = read_file();
    let ciphertext = general_purpose::STANDARD.decode(&my_str).unwrap();


    // Define the AES key (128 bits)
    let key = "YELLOW SUBMARINE".as_bytes();

    // Create an AES decryptor with ECB mode and no padding
    let mut decryptor = aes::ecb_decryptor(KeySize::KeySize128, key, NoPadding);

    let mut buffer = [0; 16];
    let mut decrypted = Vec::new();

    // Decrypt the ciphertext
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&ciphertext);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        decrypted.extend(write_buffer.take_read_buffer().take_remaining().iter());

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

   
    let decrypted_text = String::from_utf8(decrypted).expect("Invalid UTF-8");

    
    let mut output_file = File::create("decrypted.txt").expect("Failed to create output file");
    output_file.write_all(decrypted_text.as_bytes()).expect("Failed to write output file");
    
    println!("Decrypted text: {}", decrypted_text);
}


fn read_file() -> String{
    let file = File::open("encrypted.txt").unwrap();
    let mut base64_file = String::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_content: String = line.unwrap();
        base64_file += &line_content;
    }
   
    base64_file
    
}