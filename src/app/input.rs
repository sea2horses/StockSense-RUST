use owo_colors::{OwoColorize, colors::*};
use std::{
    io::{self, Read, Write},
    str::FromStr,
};

pub fn read_string(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok();

    let mut line = String::new();
    if let Err(err) = io::stdin().read_line(&mut line) {
        eprintln!("Failed to read input: {err}");
        return String::new();
    }
    line.trim().to_string()
}

pub fn read_optional_string(prompt: &str) -> Option<String> {
    let str = read_string(prompt);
    if !str.is_empty() { Some(str) } else { None }
}

pub fn read<T: FromStr>(prompt: &str, error_message: &str) -> T {
    loop {
        match read_string(prompt).parse::<T>() {
            Ok(obj) => return obj,
            Err(_) => {
                println!("{}", error_message.fg::<Red>())
            }
        };
    }
}

pub fn read_optional<T: FromStr>(prompt: &str, error_message: &str) -> Option<T> {
    loop {
        let line = read_optional_string(prompt);

        if let Some(line) = line {
            match line.parse::<T>() {
                Ok(obj) => return Some(obj),
                Err(_) => {
                    println!("{}", error_message.fg::<Red>())
                }
            }
        } else {
            return None;
        }
    }
}

pub fn halt_until_enter() {
    println!("Press enter to continue...");
    io::stdout().flush().ok();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
