use base64::{Engine as _, engine::general_purpose};


fn main() {

    let orig: &[u8; 4] = b"data";
    let decoded:Vec<u8>  = general_purpose::STANDARD_NO_PAD.decode(orig).unwrap();
    println!("{:?}", decoded);
}




// fn hamming_distance(a:&[u8], b:&[u8]) ->  Option<Vec<u8>> {
//     if a.len() != b.len() {
//         return None;
//     }

//     let mut result = Vec::with_capacity(a.len());

//     for (x, y) in a.iter().zip(b.iter()){
//         result.push(x ^ y);
//     }

//     Some(result)

// }
