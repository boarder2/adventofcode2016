use day2::input;
use std::collections::HashMap;

pub fn run() {
    // Should re-implement these using a multi dimensional array or something.
    let input = input::get_input();
    let instructions = input.split_whitespace();
    let mut numbers = HashMap::new();
    numbers.insert(1, init_direcitons(1, 2, 4, 1));
    numbers.insert(2, init_direcitons(2, 3, 5, 1));
    numbers.insert(3, init_direcitons(3, 3, 6, 2));
    numbers.insert(4, init_direcitons(1, 5, 7, 4));
    numbers.insert(5, init_direcitons(2, 6, 8, 4));
    numbers.insert(6, init_direcitons(3, 6, 9, 5));
    numbers.insert(7, init_direcitons(4, 8, 7, 7));
    numbers.insert(8, init_direcitons(5, 9, 8, 7));
    numbers.insert(9, init_direcitons(6, 9, 9, 8));
    let mut location = 5;

    print!("Day 2 Part 1: ");
    for line in instructions {
        for ch in line.chars() {
            if let Some(ref possibleLocations) = numbers.get(&location) {
                if let Some(newLocation) = possibleLocations.get(&ch) {
                    location = *newLocation;
                }
            }
        }
        print!("{}", location);
    }
    println!("");
}

fn init_direcitons(u: i32, r: i32, d: i32, l: i32) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    map.insert('U', u);
    map.insert('R', r);
    map.insert('D', d);
    map.insert('L', l);
    map
}