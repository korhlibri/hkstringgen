use clap::Parser;
use rand_core::{RngCore, OsRng};
use std::{str, process};

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

    if !args.numbers && !args.lowercase && !args.uppercase && !args.special {
        eprintln!("Error: No character type was selected to generate. Please use --help and use the wanted flags to generate respective characters.");
        process::exit(1);
    }

    let mut seconds_for_mouse: u8 = 5;

    let mut result = vec![0u8; args.range.into()];
    match OsRng.try_fill_bytes(&mut result) {
        Ok(b) => b,
        Err(_) => seconds_for_mouse = 10,
    };

    bytes_to_utfchars(&mut result, args.numbers, args.lowercase, args.uppercase, args.special);

    let result_str = match str::from_utf8(&result) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}.", e);
            process::exit(1);
        }
    };

    println!("{}", result_str);
}

fn bytes_to_utfchars(result: &mut Vec<u8>, numop: bool, lowop: bool, uppop: bool, specop: bool){
    const NUMBERS: &str = "0123456789";
    const LOWERLETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERLETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const SPECIALCHARS: &str = " -~!@#$%^&*_+=`|(){}[:;\"'<>,.?]";

    let mut range: u8 = 0;
    let mut validchars: Vec<u8> = Vec::new();
    if numop {
        range += NUMBERS.len() as u8;
        validchars.extend(NUMBERS.as_bytes().to_vec());
    }
    if lowop {
        range += LOWERLETTERS.len() as u8;
        validchars.extend(LOWERLETTERS.as_bytes().to_vec());
    }
    if uppop {
        range += UPPERLETTERS.len() as u8;
        validchars.extend(UPPERLETTERS.as_bytes().to_vec());
    }
    if specop {
        range += SPECIALCHARS.len() as u8;
        validchars.extend(SPECIALCHARS.as_bytes().to_vec());
    }

    for byte in result {
        *byte %= range;
        *byte = validchars[*byte as usize];
    }
}