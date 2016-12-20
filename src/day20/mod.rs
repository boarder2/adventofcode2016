use std::cmp;
mod input;
const RANGE_MAX: u32 = 4294967295;

pub fn run() {
	let mut first_entry: i64 = -1;
	let mut count_allowed: u32 = 0;
	let mut sorted = input::get_input()
		.lines()
		.collect::<Vec<_>>()
		.iter()
		.map(|l| {
			let a: Vec<_> = l.split('-').collect();
			(a[0].parse::<u32>().unwrap(), a[1].parse::<u32>().unwrap())
		})
		.collect::<Vec<(u32, u32)>>();
	sorted.sort_by_key(|x| x.0);

	// Ideally use references so we're not copying but meh.
	let mut largest_entry = sorted[0];
	for entry in sorted {
		if entry.1 > largest_entry.1 && entry.0 <= largest_entry.1 {
			largest_entry = entry;
		} else if entry.0 > largest_entry.1 {
			if first_entry < 0 {
				first_entry = (largest_entry.1 + 1) as i64;
			}
			count_allowed += cmp::max((entry.0 - largest_entry.1) - 1, 0);
			largest_entry = entry;
		}
	}
	count_allowed += cmp::max((RANGE_MAX - cmp::min(largest_entry.1, RANGE_MAX)), 1) - 1;
	println!("Day 20 Part 1: {:?}", first_entry);
	println!("Day 20 Part 2: {:?}", count_allowed);
}
