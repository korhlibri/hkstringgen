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

#![windows_subsystem = "windows"]

use rand_core::{RngCore, OsRng};
use std::{str, thread, time};
use mouse_position::mouse_position::Mouse;
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, ApplicationWindow, Button, Orientation, CheckButton, Entry, Label, Window, AlertDialog, Spinner, PasswordEntry
};

const APP_ID: &str = "org.gtk_rs.hkstringgen";

fn main() -> glib::ExitCode{
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    // Rust currently does not have an easy way of writing to clipboard, and most libraries
    // are either OS specific or not working. For now, easiest way is to directly ^C and ^V
    let string_display = PasswordEntry::builder()
        .editable(false)
        .show_peek_icon(true)
        .build();

    let number_check = CheckButton::builder()
        .label("Include numbers")
        .build();

    let range_entry = Entry::builder()
        .max_length(3)
        .max_width_chars(3)
        .text("8")
        .build();

    let range_label = Label::builder()
        .label("Number of characters to generate (from 1 to 255)")
        .build();

    let range_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .build();
    
    range_box.append(&range_entry);
    range_box.append(&range_label);

    let lower_check = CheckButton::builder()
        .label("Include lowercase letters")
        .build();

    let upper_check = CheckButton::builder()
        .label("Include uppercase letters")
        .build();

    let spec_check = CheckButton::builder()
        .label("Include special characters")
        .build();

    let mouse_check = CheckButton::builder()
        .label("Include mouse movement for randomization")
        .build();

    let generate_button = Button::builder()
        .label("Generate random string")
        .build();

    let loading_spinner = Spinner::builder()
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();

    gtk_box.append(&string_display);
    gtk_box.append(&range_box);
    gtk_box.append(&number_check);
    gtk_box.append(&lower_check);
    gtk_box.append(&upper_check);
    gtk_box.append(&spec_check);
    gtk_box.append(&mouse_check);
    gtk_box.append(&generate_button);
    gtk_box.append(&loading_spinner);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("hkstringgen")
        .child(&gtk_box)
        .build();

    let alert = AlertDialog::builder()
        .modal(true)
        .build();

    generate_button.connect_clicked(move |generate_button| {
        let range: u8 = match range_entry.text().as_str().parse() {
            Ok(v) => v,
            Err(_) => 0,
        };
        if !number_check.is_active() && !lower_check.is_active() && !upper_check.is_active() && !spec_check.is_active() {
            alert.set_detail("You need to specify a character type to generate");
            alert.set_message("Error");
            alert.show(None::<&Window>);
        }else if range == 0 {
            alert.set_detail("You need to specify a valid range");
            alert.set_message("Error");
            alert.show(None::<&Window>);
        }else {
            let mut seconds_for_mouse: u8 = 10;

            // Gets randomness from system (supposed to be cryptographically safe)
            let mut result = vec![0u8; range.into()];
            // If failing to get randomness from system, force the user to use mouse coordinates
            // Also doubles the seconds required to generate randomness
            match OsRng.try_fill_bytes(&mut result) {
                Ok(b) => b,
                Err(_) => seconds_for_mouse = 20,
            };
            if mouse_check.is_active() || seconds_for_mouse == 20 {
                mouse_coords_to_random(&mut result, seconds_for_mouse, 8, &alert);
            }

            bytes_to_utfchars(&mut result, number_check.is_active(), lower_check.is_active(), upper_check.is_active(), spec_check.is_active());

            // Converts utf8 bytes to readable characters
            let result_str = match str::from_utf8(&result) {
                Ok(s) => s,
                Err(_) => {
                    alert.set_message("Failed to convert bytes to string");
                    alert.set_detail("Error");
                    alert.show(None::<&Window>);
                    "0"
                },
            };
            string_display.set_text(result_str);
        }
    });

    window.present();
}

fn mouse_coords_to_random(result: &mut Vec<u8>, seconds: u8, vecrange: u8, alert: &AlertDialog){
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
                alert.set_message("Failed to get mouse position");
                alert.set_detail("Error");
                alert.show(None::<&Window>);
                return;
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