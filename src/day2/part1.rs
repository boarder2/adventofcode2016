use day2::input;
use std::cmp;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split_whitespace();
    let keypad = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let mut x: i32 = 1;
    let mut y: i32 = 1;

    print!("Day 2 Part 1: ");
    for line in instructions {
        for ch in line.chars() {
            match ch {
                'U' => y = cmp::max(y - 1, 0),
                'D' => y = cmp::min(y + 1, 2),
                'L' => x = cmp::max(x - 1, 0),
                _ => x = cmp::min(x + 1, 2),
            }
        }
        print!("{}", keypad[y as usize][x as usize]);
    }
    println!("");
}