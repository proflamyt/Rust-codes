// bruteforce all single bytes and xor with provided string 

fn main() {
    let hex_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let bytes = hex_string_to_bytes(&hex_string);
    bruteforce_xor(&bytes);
}



fn bruteforce_xor(s: &Vec<u8>)  {
    for byte_value in 0..=255 {
        let byte = byte_value as u8;
        let mut xor_result_string = String::new();
        for ch in s {
            let res = (ch ^ byte) as char;
            xor_result_string.push(res);
        }
        if is_english_like(&xor_result_string) {
            println!("Result Hex {}: {}", byte, xor_result_string);
        }
        
    }
}

fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).expect("Invalid hex"))
        .collect()
}

fn is_english_like(text: &str) -> bool {
    // Expected character frequencies for English text (approximate)
    let expected_frequencies: [f64; 26] = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
        0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
        0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
    ];

    let text = text.to_ascii_lowercase();
    let mut observed_frequencies = [0; 26];

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c as u8 - b'a') as usize;
            observed_frequencies[index] += 1;
        }
    }

    let total_chars = observed_frequencies.iter().sum::<usize>() as f64;
    let chi_squared = observed_frequencies
        .iter()
        .enumerate()
        .map(|(i, &observed)| {
            let expected = total_chars * expected_frequencies[i];
            ((observed as f64 - expected).powi(2)) / expected
        })
        .sum::<f64>();

    // Chi-squared test for goodness of fit with 25 degrees of freedom
    // Adjust the threshold as needed based on experimentation
    chi_squared < 40.0
}