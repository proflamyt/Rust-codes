fn main() {
    let mut example = String::from("YELLOW SUBMARINE");
    let ret_string =  pad(example.len(), 20);
    example.push_str(&ret_string);
    println!("{}", example);

}



fn pad(str_len: usize, pad_lenght: usize) -> String  {

    assert!(pad_lenght > str_len);

    let diffrence =  pad_lenght - str_len;

    let hex_string = format!("\\x{:02x}", diffrence); 

    hex_string.repeat(diffrence)
}