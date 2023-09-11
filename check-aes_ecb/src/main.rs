use reqwest;
extern crate hex;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let mut info: HashMap<String, i32> = HashMap::new();
    

    let cipher = fetch_from_url("https://cryptopals.com/static/challenge-data/8.txt").await?;

    for line in cipher.lines() {
        info.insert(line.to_string(), 0);

        match hex::decode(line) {
            Ok(byte_vec) => {
                let block_byte = split_blocks(&byte_vec);
                println!("Hex to Byte: {:?}", block_byte);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
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

fn check_ecb( block_bytes :Vec<&[u8]> ) {

    for blocks in block_bytes {

    }

}

async fn fetch_from_url(url: &str)  -> Result<String, reqwest::Error> {

    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    // println!("{}", body);
    Ok(body)

}