// use base64::{Engine as _, engine::general_purpose};

use std::u8;


fn main() {

    // let orig: &[u8; 4] = b"data";
    // let decoded:Vec<u8>  = general_purpose::STANDARD_NO_PAD.decode(orig).unwrap();
    // println!("{:?}", decoded);
    let my_string: String = String::from("this is a test");
    let sec_string: String =  String::from("wokka wokka!!!");
    let c: Option<u32> =  hamming_distance(my_string.as_bytes(), sec_string.as_bytes());

    match c {
        Some(distance) => {
            // Convert the Vec<u8> to a hexadecimal string
            
            println!("Hamming distance is ...: {}", distance);
        }
        None => println!("Error: Vectors must have equal lengths to compute XOR."),
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
