pub fn run() {
	let row_count_p1 = 40;
	let row_count_p2 = 400000;
	let data = String::from("^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^")
		.chars()
		.map(|c| c == '^')
		.collect::<Vec<bool>>();

	let initial_count = data.iter()
            .map(|&y| if y { 0 } else { 1 })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();
	
	println!("Day 18 Part 1: {:?}", get_count(data.clone(), initial_count, row_count_p1));
	println!("Day 18 Part 2: {:?}", get_count(data, initial_count, row_count_p2));
}

fn get_count(input: Vec<bool>, initial_count: u32, rows: u32) -> u32 {
	let mut count = initial_count;
	let mut data = input;
	for _ in 0..rows - 1 {
		let result = generate_line(data);
		data = result.0;
		count += result.1;
	}
	count
}

fn generate_line(d: Vec<bool>) -> (Vec<bool>, u32) {
	let mut new_line = Vec::with_capacity(d.len());
	let mut count = 0;
	for i in 0..d.len() {
		let t;
		if i == 0 {
			t = is_trap(false, d[i], d[i + 1]);
		} else if i == d.len() - 1 {
			t = is_trap(d[i - 1], d[i], false);
		} else {
			t = is_trap(d[i - 1], d[i], d[i + 1]);
		}
		if !t { count += 1; }
		new_line.push(t);
	}
	(new_line, count)
}

fn is_trap(l: bool, m: bool, r: bool) -> bool {
	(l && m && !r) || (!l && m && r) || (l && !m && !r) || (!l && !m && r)
}

#[test]
fn test_trap() {
	assert_eq!(is_trap(false, false, false), false);
	assert_eq!(is_trap(true, false, false), true);
	assert_eq!(is_trap(false, true, false), false);
	assert_eq!(is_trap(false, false, true), true);
	assert_eq!(is_trap(true, true, false), true);
	assert_eq!(is_trap(false, true, true), true);
	assert_eq!(is_trap(true, true, true), false);
}