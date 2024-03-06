use std::io::{self, BufRead};

fn farsi_to_english(input: &str) -> String {
    let farsi_keys: Vec<u16> = "ضصثقفغعهخحجچشسیبلاتنمکگظطزرذدپو./۱۲۳۴۵۶۷۸۹۰".encode_utf16().collect();
    let english_keys: Vec<u16> = "qwertyuiop[]asdfghjkl;'zxcvbnm,./1234567890".encode_utf16().collect();

    let mut result: Vec<u16> = vec![];

    for c in input.encode_utf16() {
        if let Some(index) = farsi_keys.iter().position(|&r| r == c) {
            result.push(english_keys[index]);
        } else {
            result.push(c);
        }
    }

    String::from_utf16(&result).expect("Conversion to string failed")
}

fn main() -> std::io::Result<()>{
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next(){
        println!("{}", farsi_to_english(&line?));
    }
    Ok(())
}
