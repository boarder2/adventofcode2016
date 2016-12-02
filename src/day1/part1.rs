use day1::input;

pub fn run() {
    let input = input::get_input();
    let instructions = input.split(", ");
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = 0; //0 North, 1 East, 2 South, 3 West
    for instruction in instructions {
        let (directionString, lengthString) = instruction.split_at(1);
        if directionString == "L" {
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
        let moveLength: i32 = lengthString.parse().expect("Not a number length");
        match direction {
            0 => y = y + moveLength,
            1 => x = x + moveLength,
            2 => y = y - moveLength,
            3 => x = x - moveLength,
            _ => println!("Busted direction."),
        }
    }
    println!("Day 1 Part 1: {}", x.abs() + y.abs());
}