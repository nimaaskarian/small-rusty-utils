use std::io::BufRead;
use std::io;

fn main() -> std::io::Result<()>{
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next(){
        let converted_string = replace_with_percentage(replace_persian_numbers(line?));
        println!("{}",converted_string);
    }
    Ok(())
}

fn replace_with_percentage(mut line: String) -> String {
    const REPLACE_WITH_PERCENT_SIGN: &str = " کی";

    for ch in REPLACE_WITH_PERCENT_SIGN.chars() {
        line = line.replace(ch, "%")
    }
    format!("%{}%", line)
}

fn replace_persian_numbers(input: String) -> String {
    let persian_numbers = vec![
        ('۰', '0'),
        ('۱', '1'),
        ('۲', '2'),
        ('۳', '3'),
        ('۴', '4'),
        ('۵', '5'),
        ('۶', '6'),
        ('۷', '7'),
        ('۸', '8'),
        ('۹', '9'),
    ];

    let mut result = String::new();
    for c in input.chars() {
        if let Some(replacement) = persian_numbers.iter().find(|&&(old, _)| old == c) {
            result.push(replacement.1);
        } else {
            result.push(c);
        }
    }
    result
}
