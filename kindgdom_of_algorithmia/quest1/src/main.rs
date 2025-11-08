use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Incorrect usage. Usage: {} <data-file-path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let battle = fs::read_to_string(file_path).expect("Failed to read file.");

    let mut num_potions: u64 = 0;
    for char in battle.chars() {
        num_potions += match char.to_ascii_lowercase() {
            'b' => 1,
            'c' => 3,
            _ => 0,
        };
    }

    println!("Number of potions needed: {}", num_potions);
}
