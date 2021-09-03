use core::time;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::{io, thread};

pub struct Util;
impl Util {
    pub fn shuffle_vec<T: std::clone::Clone>(mut items: Vec<T>) -> Vec<T> {
        let slice = items.as_mut_slice();
        let mut rng = thread_rng();
        slice.shuffle(&mut rng);

        slice.to_vec()
    }

    pub fn print_wizard_ascii_art() {
        println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
    }

    pub fn cli_next_string() -> String {
        let mut buffer = String::new();
        loop {
            io::stdin().read_line(&mut buffer).unwrap();
            match buffer.trim().parse::<String>() {
                Ok(input) => {
                    if !input.is_empty() {
                        return input;
                    }
                }
                Err(_) => {}
            }
        }
    }

    pub fn cli_next_num() -> u8 {
        loop {
            match Util::cli_next_string().parse::<u8>() {
                Ok(num) => {
                    return num;
                }
                Err(_) => {
                    println!(" * Input must be a whole number * ");
                }
            }
        }
    }

    pub fn cli_next_pos_num() -> u8 {
        loop {
            let num = Util::cli_next_num();
            if num == 0 {
                println!(" * Input must be a positive number * ");
                continue;
            }
            return num;
        }
    }

    pub fn press_enter_to_(verb: &str) {
        println!("\nPress Enter to {}...", verb);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    }

    pub fn sleep() {
        thread::sleep(time::Duration::from_millis(500));
    }
}
