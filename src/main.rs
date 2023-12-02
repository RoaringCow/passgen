use rand::Rng; // 0.8.5
#[allow(unused_imports)]
use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)] // you can discard this if you want.
struct Value {
   /// First number to add
   #[clap(short = 's', long = "size")]
   size: i32,
   /// Second number to add
   #[clap(short = 't', long = "type")]
   pass_type: i32,
}




fn main() {

    let value = Value::parse();

    let size = value.size;
    let pass_type = value.pass_type;
    
    println!("Password: {}", gen_pass(size, pass_type.to_string()));

    
}

#[allow(dead_code)]
fn gen_pass(len: i32, customize: String) -> String {
    let mut password_chars: Vec<char> = Vec::new();
    for character in customize.chars() {
        match character {
            '1' => password_chars.extend((97..=122).map(|c| c as u8 as char)),
            '2' => password_chars.extend((65..=90).map(|c| c as u8 as char)),
            '3' => password_chars.extend((48..=57).map(|c| c as u8 as char)),
            '4' => {
                password_chars.extend((33..=47).map(|c| c as u8 as char));
                password_chars.extend((58..=64).map(|c| c as u8 as char));
                password_chars.extend((91..=96).map(|c| c as u8 as char));
                password_chars.extend((123..=126).map(|c| c as u8 as char));
            }
            _ => {}
        }
    }
    let mut rng = rand::thread_rng();
    let password: String = (0..len).map(|_| password_chars[rng.gen_range(0..password_chars.len())]).collect();
    password
}