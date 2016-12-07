use crypto::digest::Digest;
use crypto::md5::Md5;
use day5::input;

pub fn run() {
	let input = input::get_input();
	let input_bytes = input.as_bytes();
	let mut num_found = 0;
	let mut md5 = Md5::new();
	let mut result = [0; 8];

	for i in 0..u64::max_value() {
		md5.input(input_bytes);
		md5.input(i.to_string().as_bytes());

		// Could do this with bytes instead of string, but meh.  It would be faster but that's a lot of extra code.
		let hash = md5.result_str();

		if hash.starts_with("00000") {
			let insert_index: usize = hash.chars().nth(5).unwrap().to_string().parse().unwrap_or(99);
			if insert_index < result.len() && result[insert_index] == 0 {
				let insert_char = hash.chars().nth(6).unwrap();
				result[insert_index] = insert_char as u8;
				println!("i:{} insert_index: {} char:{} hash:{}",
				         i,
				         insert_index,
				         insert_char,
				         hash);
				num_found += 1;
			}
		}
		md5.reset();
		if num_found > 7 {
			break;
		}
	}

	println!("Day 5 Part 2: {}",
	         String::from_utf8(result.to_vec()).unwrap());
}