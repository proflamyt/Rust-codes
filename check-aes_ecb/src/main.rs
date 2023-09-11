use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Hello, world!");
    let cipher = fetch_from_url("https://cryptopals.com/static/challenge-data/8.txt").await?;

    for line in cipher.lines() {
        println!("line: {}", line);
    }
    Ok(())
}

// fn split_blocks(blocks : [u8]){
//     const LEN: usize =  blocks.len();
//     if LEN % 16 != 0 {
//         return None;
//     }
//     let block_bytes = Vec::with_capacity(LEN/8);

//     for i in (0..len).step_by(8) {
//         block_bytes.push(&blocks[i..i+8]);
//     }

//     block_bytes
// }

async fn fetch_from_url(url: &str)  -> Result<String, reqwest::Error> {

    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    // println!("{}", body);
    Ok(body)

}