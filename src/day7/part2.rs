use day7::input;
use regex::Regex;
use std::option;

// This could be done with regular expressions if Rust regex supported forward looking and backtracing.  Unfortunately, it doesn't.
pub fn run() {
	let input = input::get_input();
	let mut count = 0;

	'line: for line in input.lines() {
		let mut in_bracket = false;
		for (i, current_char) in line.chars().enumerate() {
			if current_char == '[' {
				in_bracket = true;
				continue;
			} else if current_char == ']' {
				in_bracket = false;
				continue;
			}
			if in_bracket {
				continue;
			}

			if let Some(pattern) = find_aba_pattern(line, i) {
				let reverse_match = format!("{0}{1}{2}{1}{3}",
				                            r"\[\w*",
				                            pattern.0.to_string(),
				                            pattern.1.to_string(),
				                            r"\w*\]");
				let reg = Regex::new(&reverse_match).unwrap();

				if reg.is_match(line) {
					count += 1;
					continue 'line;
				}
			}
		}
	}
	println!("Day 7 Part 2: (Broken) {}", count);
}

fn find_aba_pattern(find: &str, offset: usize) -> option::Option<(char, char)> {
	if offset + 3 > find.len() {
		return None;
	}
	let mut iter = find.chars();
	let ch = iter.nth(offset).unwrap();
	let ch1 = iter.next().unwrap();

	if ch == ch1 || ch == '[' || ch == ']' || ch1 == '[' || ch1 == ']' {
		return None;
	}
	if iter.next().unwrap() == ch {
		return Some((ch, ch1));
	}
	None
}
