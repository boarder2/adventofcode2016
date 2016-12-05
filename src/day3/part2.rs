use day3::input;

pub fn run() {
	let input = input::get_input();
	let lines = input.lines();
	let mut number_of_lines = 0;
	let mut parsed_numbers = Vec::new();
	let mut count_good = 0;
	//Convert all strings to numbers, left to right, top to bottom
	for line in lines {
		let str_numbers = line.split_whitespace();
		for str_num in str_numbers {
			let num = str_num.parse::<i32>().expect("couldn't parse");
			parsed_numbers.push(num);
		}
		number_of_lines += 1;
	}

	for iter in 0..number_of_lines / 3 {
		let start_line = iter * 3;
		for i_column in 0..3 {
			let mut current_nums = Vec::new();
		
			for i_line in 0..3 {
					current_nums.push(parsed_numbers[((start_line + i_line) * 3) + i_column]);
			}

			current_nums.sort();
			if current_nums[0] + current_nums[1] > current_nums[2] {
					count_good += 1;
			}
		}
	}

	println!("Day 3 Part 2: {}", count_good);
}