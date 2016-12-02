use day1::input;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
    let input = input::get_input();
    let instructions = input.split(", ");
    let mut currentLocation = Point { x: 0, y: 0 };
    let mut direction = 0; //0 North, 1 East, 2 South, 3 West
    let mut visitedLocations = vec![Point { x: 0, y: 0 }];
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
            0 => currentLocation.y += moveLength,
            1 => currentLocation.x += moveLength,
            2 => currentLocation.y -= moveLength,
            3 => currentLocation.x -= moveLength,
            _ => println!("Busted direction."),
        }

        for cp in visitedLocations.iter() {
            if cp.x == currentLocation.x && cp.y == currentLocation.y {
                println!("Day 1 Part 2: {}",
                         currentLocation.x.abs() + currentLocation.y.abs());
                return;
            }
        }
        visitedLocations.push(currentLocation);
    }

}