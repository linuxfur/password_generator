use rand::{Rng};
use std::io;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    let mut rng_thread = rand::thread_rng();

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~"; //][}{;'"><%
    
    if args_len >= 2 {
        let password_len = String::from(&args[1]);
        let password_len: usize = password_len.trim().parse().expect("Enter a valid unsigned int.");
        let password: String = (0..password_len).map(|_| {
            let idx = rng_thread.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        }).collect();
        println!("{}", password);
    }
    else {
        let mut password_len = String::new();
        println!("Enter an unsigned integer:");
        io::stdin().read_line(&mut password_len).expect("Invalid number.");
        let password_len: usize = password_len.trim().parse().expect("Enter a valid unsigned int.");
        let password: String = (0..password_len).map(|_| {
            let idx = rng_thread.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        }).collect();
        println!("{}", password);
    }
}
