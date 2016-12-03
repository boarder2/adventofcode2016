use day2::input;
use std::cmp;

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
    let mut x: i32 = 0;
    let mut y: i32 = 3;

    print!("Day 2 Part 2: ");
    for line in instructions {
        for ch in line.chars() {
            let mut yChange = 0;
            let mut xChange = 0;
            match ch {
                'U' => yChange = -1,
                'D' => yChange = 1,
                'L' => xChange = -1,
                _ => xChange = 1,
            }
            let newX = cmp::max(cmp::min(x + xChange, keypad[0].len() as i32 - 1), 0);
            let newY = cmp::max(cmp::min(y + yChange, keypad.len() as i32 - 1), 0);
            if keypad[newY as usize][newX as usize] != '0' {
                x = newX;
                y = newY;
            }
        }
        print!("{}", keypad[y as usize][x as usize]);
    }
    println!("");
}