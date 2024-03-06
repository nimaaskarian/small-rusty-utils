use std::io::{self, BufRead};
use std::env;

const EN_KEYS: &str = "qwertyuiop[]asdfghjkl;'zxcvbnm,./1234567890";
const FA_KEYS: &str = "ضصثقفغعهخحجچشسیبلاتنمکگظطزرذدپو./۱۲۳۴۵۶۷۸۹۰";
fn farsi_to_english(input: &str, reverse: bool) -> String {
    let src: Vec<u16> = if reverse {
        EN_KEYS.encode_utf16().collect()
    } else {
        FA_KEYS.encode_utf16().collect()
    };
    let dst: Vec<u16> = if reverse {
        FA_KEYS.encode_utf16().collect()
    } else {
        EN_KEYS.encode_utf16().collect()
    };

    let mut result: Vec<u16> = vec![];

    for c in input.encode_utf16() {
        if let Some(index) = src.iter().position(|&r| r == c) {
            result.push(dst[index]);
        } else {
            result.push(c);
        }
    }

    String::from_utf16(&result).expect("Conversion to string failed")
}

fn main() -> std::io::Result<()>{

    let reverse = {
        let mut reverse = false;
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1] == String::from("-r") {
            reverse = true
        }
        reverse
    };
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next(){
        println!("{}", farsi_to_english(&line?, reverse));
    }
    Ok(())
}
