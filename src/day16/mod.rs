pub fn run() {
	let fill_length_part_1 = 272;
	let fill_length_part_2 = 35651584;
	let input = String::from("01110110101001000");

	let mut data = generate_data(input.to_string(), fill_length_part_1);
	let mut checksum = generate_checksum(data);
	println!("Day 16 Part 1: {}", checksum);

	data = generate_data(input, fill_length_part_2);
	checksum = generate_checksum(data);
	println!("Day 16 Part 2: {}", checksum);
}

fn generate_data(s: String, fill_length: usize) -> String {
	let mut data = s;
	while data.len() < fill_length {
		let mut new_data = String::with_capacity(data.len() + 1);
		new_data.push_str(&data);
		new_data.push('0');
		for ch in data.chars().rev() {
			if ch == '0' {
				new_data.push('1');
			} else {
				new_data.push('0');
			}
		}
		data = new_data;
	}
	data[0..fill_length].to_string()
}

fn generate_checksum(s: String) -> String {
	let mut iter = s.chars();
	let mut checksum = String::with_capacity(s.len() / 2);
	for _ in 0..s.len() / 2 {
		let ch1 = iter.next().unwrap();
		let ch2 = iter.next().unwrap();
		if ch1 == ch2 {
			checksum.push('1');
		} else {
			checksum.push('0');
		}
	}
	if checksum.len() % 2 == 0 {
		return generate_checksum(checksum);
	}
	checksum
}
