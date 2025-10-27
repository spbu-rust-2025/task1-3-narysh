use std::io;
use std::fs::File;
fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    input_text = input_text.trim().to_string();
    match File::open(input_text) {
        Ok(_file) => {println!("success")}
        Err(_error) => {println!("failure")}
    }
}