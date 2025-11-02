use std::io;
use std::fs::{metadata, File};
fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    input_text = input_text.trim().to_string();
    match metadata(input_text) {
        Ok(md) => {
            if md.is_file() {
                println!("success");
            } else {
                println!("failure");
            }
        }
        Err(_) => {
            println!("failure");
        }
    }
}