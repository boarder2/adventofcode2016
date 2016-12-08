use day7::input;

// This could be done with regular expressions if Rust regex supported forward looking and backtracing.  Unfortunately, it doesn't.
pub fn run() {
	let input = input::get_input();
	let mut count = 0;

	'line: for line in input.split_whitespace() {
		let mut found_abba = false;
		let mut in_bracket = false;

		for i in 0..line.len() - 3 {
			let current_char = line.chars().nth(i).unwrap();
			if current_char == '[' {
				in_bracket = true;
			} else if current_char == ']' {
				in_bracket = false;
			}

			if find_pattern(line, i) {
				if in_bracket {
					continue 'line;
				} else {
					found_abba = true;
				}
			}
		}
		if found_abba {
			count += 1;
		}
	}
	println!("Day 7 Part 1: {}", count);
}

fn find_pattern(find: &str, offset: usize) -> bool {
	if offset + 3 > find.len() {
		return false;
	}
	let mut iter = find.chars();
	let ch = iter.nth(offset).unwrap();
	let ch1 = iter.next().unwrap();
	if ch == ch1 {
		return false;
	}
	iter.next().unwrap() == ch1 && iter.next().unwrap() == ch
}