use clap::Parser;
use rand_core::{RngCore, OsRng};
use std::str;

#[derive(Parser)]
struct Args{
    #[arg(short, long, default_value_t = 8)]
    range: u8,
    #[arg(short, long, action)]
    numbers: bool,
    #[arg(short, long, action)]
    lowercase: bool,
    #[arg(short, long, action)]
    uppercase: bool,
    #[arg(short, long, action)]
    special: bool,
}

fn main() {
    let args = Args::parse();

    let mut result = vec![0u8; args.range.into()];
    OsRng.fill_bytes(&mut result);

    bytes_to_utfchars(&mut result);

    let result_str = match str::from_utf8(&result) {
        Ok(s) => s,
        Err(e) => panic!("{}",e)
    };

    println!("{}", result_str);
}

fn bytes_to_utfchars(result: &mut Vec<u8>){
    const NUMBERS: [u8; 2] = [48, 10];
    const LOWERLETTERS: [u8; 2] = [97, 26];
    const UPPERLETTERS: [u8; 2] = [65, 26];
    const SPECIALCHARS: &str = " -~!@#$%^&*_+=`|(){}[:;\"'<>,.?]";
    const SPECLEN: u8 = 31;

    let range: u8 = NUMBERS[1] + LOWERLETTERS[1] + UPPERLETTERS[1] + SPECLEN;

    for byte in result {
        *byte %= range;
        if *byte < NUMBERS[1] {
            *byte += NUMBERS[0];
        } else if *byte < NUMBERS[1] + UPPERLETTERS[1] {
            *byte += UPPERLETTERS[0] - NUMBERS[1];
        } else if *byte < NUMBERS[1] + UPPERLETTERS[1] + LOWERLETTERS[1] {
            *byte += LOWERLETTERS[0] - NUMBERS[1] - UPPERLETTERS[1];
        } else {
            *byte = SPECIALCHARS
                .chars()
                .nth((*byte % SPECLEN).into())
                .unwrap()
                as u8;
        }
    }
}