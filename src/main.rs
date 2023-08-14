use rand::Rng; // 0.8.5
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    lenght: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.lenght)
    }
}

fn gen_pass(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let password_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
    let password: String = (0..len).map(|_| password_chars[rng.gen_range(0..password_chars.len())]).collect();
    password
}

