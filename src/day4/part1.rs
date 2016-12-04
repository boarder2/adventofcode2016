use day4::input;
use regex::Regex;

pub fn run() {
    let input = input::get_input();
    let lines = input.lines();
    let mut count_good = 0;
    let reg = Regex::new(r"(?P<name>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]+)\]").unwrap();

    for line in lines {
        if reg.is_match(line) { 
            count_good += 1;
        }
    }


    println!("Day 4 Part 1: {}", count_good);
}