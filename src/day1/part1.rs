pub fn run() {
    let input = get_input();
    let instructions = input.split(", ");
    let mut x : i32 = 0;
    let mut y : i32 = 0;
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
            _ => println!("Busted direction.")
        }
    }
    println!("Day 1 Part 1: {}", x.abs() + y.abs());
}

fn get_input() -> String {
    return "L2, L3, L3, L4, R1, R2, L3, R3, R3, L1, L3, R2, R3, L3, R4, R3, R3, L1, L4, R4, L2, \
            R5, R1, L5, R1, R3, L5, R2, L2, R2, R1, L1, L3, L3, R4, R5, R4, L1, L189, L2, R2, L5, \
            R5, R45, L3, R4, R77, L1, R1, R194, R2, L5, L3, L2, L1, R5, L3, L3, L5, L5, L5, R2, \
            L1, L2, L3, R2, R5, R4, L2, R3, R5, L2, L2, R3, L3, L2, L1, L3, R5, R4, R3, R2, L1, \
            R2, L5, R4, L5, L4, R4, L2, R5, L3, L2, R4, L1, L2, R2, R3, L2, L5, R1, R1, R3, R4, \
            R1, R2, R4, R5, L3, L5, L3, L3, R5, R4, R1, L3, R1, L3, R3, R3, R3, L1, R3, R4, L5, \
            L3, L1, L5, L4, R4, R1, L4, R3, R3, R5, R4, R3, R3, L1, L2, R1, L4, L4, L3, L4, L3, \
            L5, R2, R4, L2"
        .to_string();
}