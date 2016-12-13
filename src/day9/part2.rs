use day9::input;

pub fn run() {
	let input = input::get_input();
	let mut char_iter = input.chars();
	let mut decomp_length = 0;
	let mut marker_start = -1;
	let mut input_index = 0;
	while input_index < input.len() {
		if let Some(ch) = char_iter.next() {
			let mut index_skip = 1;
			if ch == ')' {
				let local_marker = input[(marker_start + 1) as usize..input_index].to_string();
				let mut a = local_marker.split("x");
				let length: usize = a.next().unwrap().parse().unwrap();
				let multiplier: usize = a.next().unwrap().parse().unwrap();
				char_iter.nth(length - 1);
				index_skip = length + 1;
				let inside_length = parse(input[input_index + 1..input_index + 1 + length].to_string());
				decomp_length += inside_length * multiplier;
				marker_start = -1;
			} else if ch == '(' {
				marker_start = input_index as i32;
			} else if marker_start < 0 {
				decomp_length += 1;
			}
			input_index += index_skip;
		}
	}
	println!("Day 9 Part 2: {}", decomp_length);
}

fn parse(instr: String) -> usize {
	if let Some(start) = instr.find("(") {
		if let Some(end) = instr.find(")") {
			let local_marker = instr[(start + 1) as usize..end].to_string();
			let mut a = local_marker.split("x");
			let length: usize = a.next().unwrap().parse().unwrap();
			let multiplier: usize = a.next().unwrap().parse().unwrap();
			let current_len = parse(instr[end + 1..end + 1 + length].to_string()) * multiplier;
			let mut extra_len = 0;
			if end + 1 + length < instr.len() {
				 extra_len = parse(instr[end + 1 + length..instr.len()].to_string());
			}
			return current_len + extra_len;
		}
	}
	instr.len()
}