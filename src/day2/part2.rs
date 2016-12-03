use day2::input;
use std::collections::HashMap;

pub fn run() {
    // Should re-implement these using a multi dimensional array or something.
    let input = input::get_input();
    let instructions = input.split_whitespace();
    let mut numbers = HashMap::new();
    numbers.insert('1', init_direcitons('1', '1', '3', '1'));
    numbers.insert('2', init_direcitons('2', '3', '6', '2'));
    numbers.insert('3', init_direcitons('1', '4', '7', '2'));
    numbers.insert('4', init_direcitons('4', '4', '8', '3'));
    numbers.insert('5', init_direcitons('5', '6', '5', '5'));
    numbers.insert('6', init_direcitons('2', '7', 'A', '5'));
    numbers.insert('7', init_direcitons('3', '8', 'B', '6'));
    numbers.insert('8', init_direcitons('4', '9', 'C', '7'));
    numbers.insert('9', init_direcitons('9', '9', '9', '8'));
    numbers.insert('A', init_direcitons('6', 'B', 'A', 'A'));
    numbers.insert('B', init_direcitons('7', 'C', 'D', 'A'));
    numbers.insert('C', init_direcitons('8', 'C', 'C', 'B'));
    numbers.insert('D', init_direcitons('B', 'D', 'D', 'D'));

    let mut location = '5';

    print!("Day 2 Part 2: ");
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

fn init_direcitons(u: char, r: char, d: char, l: char) -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('U', u);
    map.insert('R', r);
    map.insert('D', d);
    map.insert('L', l);
    map
}