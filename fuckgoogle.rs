use std::env;
fn main() {
    let args:Vec<String> = env::args().collect();
    let google_url:&String = &args[1];

    for part in google_url.split("&") {
        let parted_part:Vec<&str> = part.split("=").collect();

        if parted_part[0] == "url" {
            println!("{}", parted_part[1]);
            break;
        }
    }
}
