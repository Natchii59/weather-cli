use colored::*;
use std::io;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt.bright_green());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
