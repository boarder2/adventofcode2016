use day4::input;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Entry<'a> {
	name: &'a str,
	sector: i32,
	checksum: &'a str,
}

pub fn run() {
	let input = input::get_input();
	let lines = input.lines();
	let mut sector_sum = 0;
	let reg = Regex::new(r"(?P<name>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]+)\]").unwrap();
	let mut entries = Vec::new();

	for line in lines {
		let capture = reg.captures(line).unwrap();
		let e = Entry {
			name: capture.name("name").unwrap(),
			sector: capture.name("sector").unwrap().parse().unwrap(),
			checksum: capture.name("checksum").unwrap(),
		};
		entries.push(e);
	}

	'entries_loop: for e in entries {
		let mut count_chars: HashMap<char, i32> = HashMap::new();
		for ch in e.name.replace("-", "").chars() {
			*count_chars.entry(ch).or_insert(0) += 1;
		}

		let mut count_vec: Vec<_> = count_chars.iter().collect();
		count_vec.sort_by(|a, b| {
			let count_order = b.1.cmp(a.1);
			if count_order == Ordering::Equal {
				a.0.cmp(b.0)
			} else {
				count_order
			}
		});

		for (i, ch) in e.checksum.chars().enumerate() {
			if ch != *count_vec[i].0 {
				continue 'entries_loop;
			}
		}

		sector_sum += e.sector;
	}

	println!("Day 4 Part 1: {}", sector_sum);
}