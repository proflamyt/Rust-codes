use base64::{Engine as _, engine::general_purpose};
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {

    let my_str = read_file();
    let bytes = general_purpose::STANDARD.decode(&my_str).unwrap();
    let mut averages: HashMap<usize, f32> = HashMap::new();
    


    for keysize in 2..41 {

        let mut norm: Vec<u32> = Vec::new();

       // print!("Bytes  {:?}", bytes);

        let block_bytes = split_bytes_into_blocks(&bytes, keysize, true);
        let num_blocks = block_bytes.len();
        
        for i in (0..num_blocks-1).step_by(2) {

            let c: Option<u32> =  hamming_distance(&block_bytes[i], &block_bytes[i+1]);
            
            match c {
                Some(distance) => {
                    // Convert the Vec<u8> to a hexadecimal string
                    norm.push(distance/keysize as u32);
                    // find smallest
                }
                None => println!("Error: Vectors must have equal lengths to compute XOR."),
            }
            
        }
        // print!("norms {:?}", norm);

        let sum: u32 = norm.iter().sum();
        let average: f32 = sum as f32 / norm.len() as f32;
        averages.insert(keysize, average);
        norm.clear();
        
    }

    let mut kv_pairs: Vec<_> = averages.iter().collect();


    kv_pairs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    let smallest_keys: Vec<usize> = kv_pairs.iter().take(1).map(|&(key, _)| *key).collect();

    
    for i in smallest_keys {

        let first_block_bytes: Vec<Vec<u8>> = split_bytes_into_blocks(&bytes, i , false);

        let trans_blocks = transpose_block(first_block_bytes, i );

        for j in trans_blocks {
            bruteforce_xor(&j);
        }

    }


}




fn hamming_distance(a:&[u8], b:&[u8]) ->  Option<u32> {
    assert_eq!(a.len(), b.len());

    let mut result: u32 = 0;

    for (x, y) in a.iter().zip(b.iter()){
        let xor_result = x ^ y;
        result += xor_result.count_ones();
    }
    Some(result)
}


fn read_file() -> String{
    let file = File::open("file6.txt").unwrap();
    let mut base64_file = String::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_content: String = line.unwrap();
        base64_file += &line_content;
    }
   
    base64_file
    
}


fn split_bytes_into_blocks(data: &[u8], block_size: usize, instance: bool) -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    let mut current_block = Vec::with_capacity(block_size);
    let mut count = 0;

    for &byte in data {
        current_block.push(byte);

        if current_block.len() == block_size {
            blocks.push(current_block.clone()); // Clone to create a new block
            current_block.clear();
            count += 1;
        }

        if instance == true && count == 16 {
            return blocks;
        }

    }

    // Add any remaining bytes as the last block
    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    blocks
}


fn bruteforce_xor(s: &Vec<u8>)  {

    let mut best_key = 'a';
    let mut best_score = 0;

    for byte_value in 0..=255 {
        let byte = byte_value as u8;
        

        let mut xor_result_string : Vec<u8> = Vec::new();


        for ch in s {
            let res = (ch ^ byte) as u8;
            xor_result_string.push(res);
        }

        let score = score_plaintext(&xor_result_string);
        
        if score > best_score {
            best_score = score;
            best_key = char::from(byte_value);
        } 
    }
    print!("{}", best_key);
   
}

fn score_plaintext(plaintext: &[u8]) -> usize {
    let valid_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.!?";
    let mut score = 0;

    for &byte in plaintext.iter() {
        let char = byte as char;
        if valid_chars.contains(char) {
            score += 1;
        }
    }

    score
}


fn transpose_block(block_bytes:Vec<Vec<u8>> , key_size: usize) -> Vec<Vec<u8>> {

    let mut blocks: Vec<Vec<_>> = Vec::new();
    let mut trans_block = Vec::with_capacity(key_size);

    for j in 0..key_size {
            for block in &block_bytes {
                match block.get(j) {
                    Some(value) => trans_block.push(*value),
                    None => break,
                }    
                
            }
        blocks.push(trans_block.clone());
        trans_block.clear();
    }    
    blocks
}    
