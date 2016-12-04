use day1::input;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split(", ");
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = 0; //0 North, 1 East, 2 South, 3 West
    for instruction in instructions {
        let (direction_string, length_string) = instruction.split_at(1);
        if direction_string == "L" {
            direction = direction - 1;
            if direction < 0 {
                direction = 3;
            }
        } else {
            direction = direction + 1;
            if direction > 3 {
                direction = 0;
            }
        }
        let move_length: i32 = length_string.parse().expect("Not a number length");
        match direction {
            0 => y = y + move_length,
            1 => x = x + move_length,
            2 => y = y - move_length,
            3 => x = x - move_length,
            _ => println!("Busted direction."),
        }
    }
    println!("Day 1 Part 1: {}", x.abs() + y.abs());
}