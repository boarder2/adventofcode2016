use day4::input;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Entry<'a> {
	name: &'a str,
	sector: i32,
	checksum: &'a str,
}

pub fn run() {
	let input = input::get_input();
	let lines = input.lines();
	let mut count_good = 0;
	let reg = Regex::new(r"(?P<name>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]+)\]").unwrap();
	let mut entries = Vec::new();

	for line in lines {
		let capture = reg.captures(line).unwrap();
		let e = Entry {
			name: capture.name("name").unwrap(),
			sector: capture.name("sector").unwrap().parse().expect("couldn't parse sector"),
			checksum: capture.name("checksum").unwrap(),
		};
		entries.push(e);
	}

	for e in entries {
		println!("Name: {} Sector: {} Checksum: {}",
		         e.name.replace("-", ""),
		         e.sector,
		         e.checksum);
	}

	println!("Day 4 Part 1: {}", count_good);
}