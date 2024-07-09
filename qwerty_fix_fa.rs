use std::io::{self, BufRead};
use std::env;

const EN_KEYS: &str = "qwertyuiop[]asdfghjkl;'zxcvbnm,./1234567890`~\"";
const FA_KEYS: &str = "ضصثقفغعهخحجچشسیبلاتنمکگظطزرذدپو./۱۲۳۴۵۶۷۸۹۰‍÷؛";
type Char = char;

#[inline]
fn src_dst_fa_en(reverse: bool) -> (Vec<Char>, Vec<Char>){
    let src: Vec<Char> = if reverse {
        EN_KEYS
    } else {
        FA_KEYS
    }.chars().collect();
    let dst: Vec<Char> = if reverse {
        FA_KEYS
    } else {
        EN_KEYS
    }.chars().collect();
    (src, dst)
}

fn src_to_dst_in_input(input: &str, src: &[Char], dst: &[Char]) -> String {
    let mut result: Vec<Char> = vec![];

    for c in input.chars() {
        if let Some(index) = src.iter().position(|&r| r == c) {
            result.push(dst[index]);
        } else {
            result.push(c);
        }
    }

    result.into_iter().collect()
}

fn main() -> std::io::Result<()>{

    let reverse = {
        let mut reverse = false;
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1].as_str() == "-r" {
            reverse = true
        }
        reverse
    };

    let (src, dst) = src_dst_fa_en(reverse);
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next(){
        println!("{}", src_to_dst_in_input(&line?, &src, &dst));
    }
    Ok(())
}
