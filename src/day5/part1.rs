use crypto::digest::Digest;
use crypto::md5::Md5;
use day5::input;

pub fn run() {
	let input = input::get_input();
	let input_bytes = input.as_bytes();
	let mut num_found = 0;
	let mut md5 = Md5::new();
	let mut result = String::with_capacity(8);

	for i in 0..u64::max_value() {
		md5.input(input_bytes);
		md5.input(i.to_string().as_bytes());
		
		//Could do this with bytes instead of string, but meh.  It would be faster but that's a lot of extra code.
		let hash = md5.result_str();

		if hash.starts_with("00000") {
			let sixth = hash.chars().nth(5).unwrap();
			println!("i:{} sixth: {} hash:{}", i, sixth, hash);
			result.push(sixth);
			num_found += 1;
		}
		md5.reset();
		if num_found > 7 {
			break;
		}
	}

	println!("Day 5 Part 1: {}", result);
}