use day2::input;
use std::cmp;
use utils;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split_whitespace();
    let keypad = [
        ['0', '0', '1', '0', '0'],
        ['0', '2', '3', '4', '0'],
        ['5', '6', '7', '8', '9'],
        ['0', 'A', 'B', 'C', '0'],
        ['0', '0', 'D', '0', '0']
    ];
    let mut loc = utils::Point { x: 0, y: 3};

    print!("Day 2 Part 2: ");
    for line in instructions {
        for ch in line.chars() {
            let mut y_change = 0;
            let mut x_change = 0;
            match ch {
                'U' => y_change = -1,
                'D' => y_change = 1,
                'L' => x_change = -1,
                _ => x_change = 1,
            }
            let new_x = cmp::max(cmp::min(loc.x + x_change, keypad[0].len() as i32 - 1), 0);
            let new_y = cmp::max(cmp::min(loc.y + y_change, keypad.len() as i32 - 1), 0);
            if keypad[new_y as usize][new_x as usize] != '0' {
                loc.x = new_x;
                loc.y = new_y;
            }
        }
        print!("{}", keypad[loc.y as usize][loc.x as usize]);
    }
    println!("");
}