use std::cmp;
mod input;

#[derive(Debug, Copy, Clone)]
struct S {
	min: u32,
	max: u32,
}

pub fn run() {
	let input = input::get_input();
	let mut first_entry: i64 = -1;
	let mut count_allowed: u32 = 0;
	let mut sorted = input.lines()
		.collect::<Vec<_>>()
		.iter()
		.map(|l| {
			let mut a = l.split('-');
			let min: u32 = a.next().unwrap().parse().unwrap();
			let max: u32 = a.next().unwrap().parse().unwrap();
			let s = S {
				min: min,
				max: max,
			};
			s
		})
		.collect::<Vec<_>>();
	sorted.sort_by_key(|x| x.min);

	//Ideally use references so we're not copying but meh.
	let mut largest_entry = sorted[0];
	for entry in sorted {
		if entry.max > largest_entry.max && entry.min < largest_entry.max {
			largest_entry = entry;
		} else if (entry.min == largest_entry.min && entry.max == largest_entry.max) ||
		          (entry.min < largest_entry.max && entry.max < largest_entry.max) {
			continue;
		} else if entry.min > largest_entry.max {
			if first_entry < 0 {
				first_entry = (largest_entry.max + 1) as i64;
			}
			count_allowed += cmp::max((entry.min - largest_entry.max) - 1, 0);
			largest_entry = entry;
		}
	}
	count_allowed += cmp::max((4294967295 - cmp::min(largest_entry.max, 4294967295)), 1) - 1;
	println!("Day 20 Part 1: {:?}", first_entry);
	println!("Day 20 Part 2: {:?}", count_allowed);
}
