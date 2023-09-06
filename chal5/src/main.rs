use base64::{Engine as _, engine::general_purpose};
use std::fs::File;
use std::io::{self, BufRead};


fn main() {

    let my_str = read_file();
    let bytes = general_purpose::STANDARD.decode(&my_str).unwrap();
    let mut averages =  Vec::new();
    


    for keysize in 2..40 {

        if keysize > my_str.len() || 2*keysize > my_str.len() {
            break;
        }

        let mut norm: Vec<u32> = Vec::new();

        let block_bytes = split_bytes_into_blocks(&bytes, keysize, true);

        for i in 0..(block_bytes.len()-1) {
            let c: Option<u32> =  hamming_distance(&block_bytes[i], &block_bytes[i+1]);
            match c {
                Some(distance) => {
                    // Convert the Vec<u8> to a hexadecimal string
                    
                    let normalized: u32 = distance;
                    norm.push(normalized);
    
                    // find smallest
                    
                }
                None => println!("Error: Vectors must have equal lengths to compute XOR."),
            }
        }
        let sum: u32 = norm.iter().sum();
        let average: u32 = sum / norm.len() as u32;
        averages.push(average);

        norm.clear();
        
    }
    
    

    if let Some((min_index, _min_value)) = &averages.iter().enumerate().min_by_key(|(_, &x)| x) {
        let key_size = min_index + 2;
        let block_bytes: Vec<Vec<u8>> = split_bytes_into_blocks(&bytes, key_size, false);

        let mut blocks = Vec::new();
        
    
        for j in 0..key_size {
            let mut trans_block = Vec::with_capacity(key_size);

            for block in &block_bytes {
                trans_block.push(block[j]);
            }
            bruteforce_xor(&trans_block);
            blocks.push(trans_block);
        }
        
    } else {
        println!("The vector is empty.");
    }

   

}




fn hamming_distance(a:&[u8], b:&[u8]) ->  Option<u32> {
    if a.len() != b.len() {
        return None;
    }

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

        if instance == true && count == 4 {
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
    for byte_value in 0..=128 {
        let byte = byte_value as u8;
        let mut xor_result_string : Vec<u8> = Vec::new();
        for ch in s {
            let res = (ch ^ byte) as u8;
            xor_result_string.push(res);
        }
        let decoded_str = String::from_utf8_lossy(&xor_result_string);

        if decoded_str.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
            println!(" Decoded: {}",  decoded_str);
        }
    }
}