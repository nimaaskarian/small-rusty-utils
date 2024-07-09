use std::io::{self, BufRead};
use std::env;

const EN_KEYS: &[char] = &['q','w','e','r','t','y','u','i','o','p','[',']','a','s','d','f','g','h','j','k','l',';', '\'',
                            'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '1', '2', '3', '4', '5', '6',
                            '7', '8', '9', '0', '`', '~'];

const FA_KEYS: &[char] = &['ض' ,'ص' ,'ث' ,'ق' ,'ف' ,'غ' ,'ع' ,'ه' ,'خ' ,'ح' ,'ج' ,'چ' ,'ش' ,
                            'س' ,'ی' ,'ب' ,'ل' ,'ا' ,'ت' ,  'ن',  'م' , 'ک' , 'گ' , 'ظ' ,
                            'ط' , 'ز' , 'ر' , 'ذ' , 'د' , 'پ',  'و',  '.', '/',  '۱', '۲', '۳',
                            '۴', '۵', '۶', '۷', '۸', '۹', '۰', '\u{200D}', '\u{00F7}', '؛'];

#[inline(always)]
fn src_dst_fa_en(reverse: bool) -> (&'static [char], &'static [char]){
    if reverse {
        (EN_KEYS, FA_KEYS)
    } else {
        (FA_KEYS, EN_KEYS)
    }
}

#[inline(always)]
fn src_to_dst_in_input(input: &str, src: &[char], dst: &[char]) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if let Some(index) = src.iter().position(|&r| r == c) {
            result.push(dst[index]);
        } else {
            result.push(c);
        }
    }

    result
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
