use clap::Parser;
use rand_core::{RngCore, OsRng};
use std::{str, process, thread, time};
use mouse_position::mouse_position::Mouse;

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
    #[arg(short, long, action)]
    mouse: bool,
}

fn main() {
    let args = Args::parse();

    if !args.numbers && !args.lowercase && !args.uppercase && !args.special {
        eprintln!("Error: No character type was selected to generate. Use --help and select the wanted flags to generate respective characters.");
        process::exit(1);
    }

    let mut seconds_for_mouse: u8 = 5;

    let mut result = vec![0u8; args.range.into()];
    match OsRng.try_fill_bytes(&mut result) {
        Ok(b) => b,
        Err(_) => seconds_for_mouse = 10,
    };

    if !args.mouse && seconds_for_mouse == 10{
        eprintln!("Error: Failed to get randomness from system. In order to generate random values, mouse movement needs to be recorded. Use --mouse to record mouse movement.");
        process::exit(1);
    } else if args.mouse {
        mouse_coords_to_random(&mut result, seconds_for_mouse, args.range)
    }

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

fn mouse_coords_to_random(result: &mut Vec<u8>, seconds: u8, vecrange: u8){
    let iter: u16 = (seconds as u16) * 200;
    let mut curriter: u16 = 0;
    let mut lastx: i32 = 0;
    let mut lasty: i32 = 0;
    let mut i: u8 = 0;
    while curriter <= iter {
        let position = Mouse::get_mouse_position();
        let (x, y) = match position {
            Mouse::Position { x, y } => (x, y),
            Mouse::Error => {
                eprintln!("Error: Failed to get mouse position.");
                process::exit(1);
            },
        };
        if x == lastx && y == lasty {
            continue;
        }
        lastx = x;
        lasty = y;
        let pos = x + y;
        result[i as usize] = result[i as usize].wrapping_add((pos % 256) as u8);
        i = (i + 1) % vecrange;
        curriter += 1;
        thread::sleep(time::Duration::from_millis(5));
    }
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