use day2::input;
use std::cmp;
use utils;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split_whitespace();
    let keypad = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let mut loc = utils::Point { x: 1, y: 1 };

    print!("Day 2 Part 1: ");
    for line in instructions {
        for ch in line.chars() {
            match ch {
                'U' => loc.y = cmp::max(loc.y - 1, 0),
                'D' => loc.y = cmp::min(loc.y + 1, keypad.len() as i32 - 1),
                'L' => loc.x = cmp::max(loc.x - 1, 0),
                _ => loc.x = cmp::min(loc.x + 1, keypad[0].len() as i32 - 1),
            }
        }
        print!("{}", keypad[loc.y as usize][loc.x as usize]);
    }
    println!("");
}