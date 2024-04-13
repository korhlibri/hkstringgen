// hkstringgen - Simple random string generator with mouse movement option and GUI
// Copyright (C) 2024 Hlib Korzhynskyy
// 
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License along with this
// program. If not, see <https://www.gnu.org/licenses/>.

use rand_core::{RngCore, OsRng};
use std::{str, process, thread, time};
use mouse_position::mouse_position::Mouse;

fn main() {
    let mut seconds_for_mouse: u8 = 10;

    // Gets randomness from system (supposed to be cryptographically safe)
    let mut result = vec![0u8; 8];
    match OsRng.try_fill_bytes(&mut result) {
        Ok(b) => b,
        Err(_) => seconds_for_mouse = 20,
    };

    // If failing to get randomness from system, force the user to use mouse coordinates
    // Also doubles the seconds required to generate randomness
    if true && seconds_for_mouse == 20{
        eprintln!("Error: Failed to get randomness from system. In order to generate random values, mouse movement needs to be recorded. Use --mouse to record mouse movement.");
        process::exit(1);
    } else if true {
        mouse_coords_to_random(&mut result, seconds_for_mouse, 8)
    }

    bytes_to_utfchars(&mut result, true, true, true, true);

    // Converts utf8 bytes to readable characters
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

        // Makes sure that the coordinates are always different
        if x == lastx && y == lasty {
            thread::sleep(time::Duration::from_millis(10));
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

    // Range of the included characters to convert
    let mut range: u8 = 0;
    // This vector will contain all the characters included by user for ease of access
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