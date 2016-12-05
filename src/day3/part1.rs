use day3::input;

pub fn run() {
	let input = input::get_input();
	let lines = input.lines();
	let mut count_good = 0;

	for line in lines {
		let str_numbers = line.split_whitespace();
		let mut parsed_numbers = Vec::new();
		for str_num in str_numbers {
			let num = str_num.parse::<i32>().expect("couldn't parse");
			parsed_numbers.push(num);
		}
		parsed_numbers.sort();
		if parsed_numbers[0] + parsed_numbers[1] > parsed_numbers[2] {
			count_good += 1;
		}
	}


	println!("Day 3 Part 1: {}", count_good);
}