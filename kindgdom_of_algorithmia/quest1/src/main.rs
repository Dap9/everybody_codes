use std::{env, fs};

fn get_potion_req(bug: char) -> u64 {
    match bug.to_ascii_lowercase() {
        'a' => 0,
        'b' => 1,
        'c' => 3,
        'd' => 5,
        _ => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Incorrect usage. Usage: {} <data-file-path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let battle = fs::read_to_string(file_path).expect("Failed to read file.");
    let battle = battle.trim();

    let mut num_potions: u64 = 0;
    for char in battle.chars() {
        num_potions += get_potion_req(char);
    }

    println!("Number of potions needed individually: {}", num_potions);

    num_potions = 0;
    let chars: Vec<char> = battle.chars().collect();
    for pair in chars.chunks(2) {
        num_potions += get_potion_req(pair[0]);
        num_potions += get_potion_req(pair[1]);
        num_potions += if pair[0] != 'x' && pair[1] != 'x' {
            2
        } else {
            0
        }
    }

    println!("Number of potions needed in pairs: {}", num_potions);

    num_potions = 0;
    let chars: Vec<char> = battle.chars().collect();
    for trip in chars.chunks(3) {
        num_potions += get_potion_req(trip[0]);
        num_potions += get_potion_req(trip[1]);
        num_potions += get_potion_req(trip[2]);
        num_potions += if trip[0] != 'x' && trip[1] != 'x' && trip[2] != 'x' {
            2 * 3
        } else if (trip[0] != 'x' && trip[1] != 'x')
            || (trip[0] != 'x' && trip[2] != 'x')
            || (trip[1] != 'x' && trip[2] != 'x')
        {
            1 * 2
        } else {
            0
        }
    }

    println!("Number of potions needed in trips: {}", num_potions);
}
