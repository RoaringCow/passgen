use rand::Rng; // 0.8.5
use std::io;

fn main() {
    println!("Enter password lenght(max i16):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    let size: i16 = input.trim().parse().expect("Invalid input");

    println!("Customize the password.\nType 1 to include lowercase letters\nType 2 to include uppercase letters\nType 3 to include numbers\nType 4 to include special characters\nUsage: 124 includes everything except numbers");
    // i know it it unreadable now but just unwrap it by the \n tags.

    let mut customize = String::new();
    io::stdin().read_line(&mut customize).expect("Failed to read the input");

    println!("{}" ,gen_pass(size, customize));
}

fn gen_pass(len: i16, customize: String) -> String {
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
