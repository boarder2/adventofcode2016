use day9::input;

pub fn run() {
	let input = input::get_input();
	let mut char_iter = input.chars();
	let mut keep_processing = true;
	let mut decomp_length = 0;
	let mut in_marker = false;
	let mut marker = String::new();
	while keep_processing {
		if let Some(ch) = char_iter.next() {
			if ch == ')' {
				in_marker = false;
				let local_marker = marker.to_owned();
				let mut a = local_marker.split("x");
				let length: usize = a.next().unwrap().parse().unwrap();
				let multiplier: usize = a.next().unwrap().parse().unwrap();
				char_iter.nth(length - 1);
				decomp_length += length * multiplier;
				marker.clear();
			} else if in_marker {
				marker.push(ch);
			} else if ch == '(' {
				in_marker = true
			} else {
				decomp_length += 1;
			}
		} else {
			keep_processing = false;
		}
	}
	println!("Day 9 Part 1: {}", decomp_length);
}