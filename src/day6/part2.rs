use day6::input;
use std::collections::HashMap;

pub fn run() {
	let input = input::get_input();
	let mut columns = Vec::new();
	let mut result = String::new();

	for _ in 0..input.split_whitespace().next().unwrap().len() {
		let map: HashMap<char, u32> = HashMap::new();
		columns.push(map);
	}

	for line in input.split_whitespace() {
		for (i, ch) in line.chars().enumerate() {
			*columns[i].entry(ch).or_insert(0) += 1;
		}
	}

	for column in columns {
		let mut map_array: Vec<_> = column.iter().collect();
		map_array.sort_by(|a, b| a.1.cmp(b.1));
		result.push(*map_array[0].0);
	}

	println!("Day 6 Part 2: {}", result);
}