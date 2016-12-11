use day9::input;
use regex::Regex;

pub fn run() {
	let input = input::get_input();
	let mut char_iter = input.chars();
	let mut keep_processing = true;
	let mut decomp_length = 0;
	let mut in_marker = false;
	let mut marker = String::new();
	let marker_rex = Regex::new(r"(?P<length>\d*)x(?P<multiplier>\d*)").unwrap();
	while keep_processing {
		if let Some(ch) = char_iter.next() {
			if ch == ')' {
				in_marker = false;
				if let Some(captures) = marker_rex.captures(&marker) {
					let length: usize = captures.name("length").unwrap().parse().unwrap();
					let multiplier: usize = captures.name("multiplier").unwrap().parse().unwrap();
					char_iter.nth(length - 1);
					decomp_length += length * multiplier;
				}
				marker.clear();
				continue;
			} else if in_marker {
				marker.push(ch);
				continue;
			} else if ch == '(' {
				in_marker = true;
				continue;
			}
			decomp_length += 1;
		} else {
			keep_processing = false;
		}		
	}
	println!("Day 9 Part 1: {}", decomp_length);
}