use day4::input;

pub fn run() {
    let input = input::get_input();
    let lines = input.lines();
    let mut count_good = 0;

    for line in lines {
        count_good += 1;
    }


    println!("Day 4 Part 1: {}", count_good);
}