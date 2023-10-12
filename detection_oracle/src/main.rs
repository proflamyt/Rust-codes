use rand::Rng;



fn main() {
    let my_string = "Olamide";
    let my_bytes: &[u8] = my_string.as_bytes();
    let _key = rand_key();
    let paded_bytes = pad_plaintext(my_bytes);
    print!("{:?}", paded_bytes);
}

fn encryption_oracle(input: String) {

    let random_bit: u8 = rand::thread_rng().gen_range(0..=1);
}



fn pad_plaintext(plaintext: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bytes = Vec::from(plaintext);

    for i in 0..2 { // Generate two random byte arrays
        let length = rng.gen_range(5..=10);
        let mut random_bytes_part = vec![0u8; length];
        rng.fill(random_bytes_part.as_mut_slice());
        if i == 0 {
            random_bytes.extend_from_slice(&random_bytes_part);
        }
        else {
            random_bytes.splice(0..0, random_bytes_part.clone());
        }
    }

    random_bytes
}


fn rand_key() -> [u8; 16] {
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
    random_bytes
}