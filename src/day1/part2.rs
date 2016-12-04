use day1::input;
use utils;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split(", ");
    let mut current_location = utils::Point { x: 0, y: 0 };
    let mut direction = 0; //0 North, 1 East, 2 South, 3 West
    let mut visited_locations = vec![utils::Point { x: 0, y: 0 }];
    for instruction in instructions {
        let (direction_string, length_string) = instruction.split_at(1);
        if direction_string == "L" {
            direction -= 1;
            if direction < 0 {
                direction = 3;
            }
        } else {
            direction += 1;
            if direction > 3 {
                direction = 0;
            }
        }
        let move_length: i32 = length_string.parse().expect("Not a number length");
        for _ in 0..move_length {
            match direction {
                0 => current_location.y += 1,
                1 => current_location.x += 1,
                2 => current_location.y -= 1,
                _ => current_location.x -= 1,
            }
            for cp in visited_locations.iter() {
                if cp.x == current_location.x && cp.y == current_location.y {
                    println!("Day 1 Part 2: {}",
                             current_location.x.abs() + current_location.y.abs());
                    return;
                }
            }
            visited_locations.push(current_location);
        }
    }
}