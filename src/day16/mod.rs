pub fn run() {
	let fill_length_part_1 = 272;
	let fill_length_part_2 = 35651584;
	let input = String::from("01110110101001000").chars().map(|c| c == '1').collect::<Vec<bool>>();

	let mut data = generate_data(input.clone(), fill_length_part_1);
	let mut checksum = generate_checksum(data);
	println!("Day 16 Part 1: {}", checksum.into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>());

	data = generate_data(input, fill_length_part_2);
	checksum = generate_checksum(data);
	println!("Day 16 Part 2: {}", checksum.into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>());
}

fn generate_data(d: Vec<bool>, fill_length: usize) -> Vec<bool> {
	let mut data = Vec::with_capacity(fill_length);
	data.extend(d);
	while data.len() < fill_length {
		let rev = data.iter().rev().map(|x| !x).collect::<Vec<bool>>();
		data.push(false);
		data.extend(rev);
	}
	data.truncate(fill_length);
	data
}

fn generate_checksum(d: Vec<bool>) -> Vec<bool> {
	let mut checksum = Vec::with_capacity(d.len() / 2);
	for chunk in d.chunks(2) {
		checksum.push(chunk[0] == chunk[1]);
	}
	if checksum.len() % 2 == 0 {
		return generate_checksum(checksum);
	}
	checksum
}
