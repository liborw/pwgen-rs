use clap::Parser;
use rand::Rng;

const UCHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LCHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "1234567890";
const SYMBOLS: &str = "";

/// Simple password generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Mode string
   #[arg(short, long, default_value_t = String::from("luns"))]
   mode: String,

   /// Number of characters in a group
   #[arg(short, long, default_value_t = 8)]
   length: u32,

   /// Number of groups in the password
   #[arg(short, long, default_value_t = 1)]
   groups: u32,
}

fn pwgen(mode: String, length: u32, groups: u32) -> String {

    let mut pool = String::new();

    for m in mode.chars() {
        match m {
            'l' => {pool.push_str(LCHARS)},
            'u' => {pool.push_str(UCHARS)},
            'n' => {pool.push_str(NUMBERS)},
            's' => {pool.push_str(SYMBOLS)},
            _ => {},
        }
    }
    let pool: Vec<char> = pool.chars().collect();
    let pool_size = pool.len();

    let mut pw = String::new();
    for g in 0..groups {

        if g >= 1 {
            pw.push('-');
        }

        for _ in 0..length {
            let ch = pool[rand::thread_rng().gen_range(0..pool_size)];
            pw.push(ch);
        }
    }

    return pw;
}

fn main() {
    let args = Args::parse();
    println!("{}", pwgen(args.mode, args.length, args.groups));
}
