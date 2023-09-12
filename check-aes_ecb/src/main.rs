use reqwest;
extern crate hex;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let mut info: HashMap<String, i32> = HashMap::new();
    

    let cipher = fetch_from_url("https://cryptopals.com/static/challenge-data/8.txt").await?;

    for line in cipher.lines() {
        
        match hex::decode(line) {
            Ok(byte_vec) => {
                let block_byte = split_blocks(&byte_vec);
                let count = check_ecb(block_byte);
                info.insert(line.to_string(), count);
                
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
    for (key, value) in info {
        if value > 0 {
            println!("Key: {}, Value: {} is greater than 0.", key, value);
        }
    }

    Ok(())
}

fn split_blocks(blocks : &[u8]) -> Vec<&[u8]>{
    let LEN: usize =  blocks.len();
    // if LEN % 16 != 0 {
    //     return None;
    // }
    let mut block_bytes = Vec::with_capacity(LEN/16);

    for i in (0..LEN).step_by(16) {
        block_bytes.push(&blocks[i..i+16]);
    }

    block_bytes
}

fn check_ecb(block_bytes :Vec<&[u8]> ) -> i32 {


    let mut count = 0;
    let mut counted =  Vec::with_capacity(16);

    for (i, blocks) in block_bytes.iter().enumerate() {

        if !counted.contains(blocks) {

        for j in i+1 .. block_bytes.len() {
            if *blocks == block_bytes[j] {
                count += 1;
            }
        }
        counted.push(blocks);
    }
    }

    count

}

async fn fetch_from_url(url: &str)  -> Result<String, reqwest::Error> {

    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    // println!("{}", body);
    Ok(body)

}