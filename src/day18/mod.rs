pub fn run() {
	let row_count_p1 = 40;
	let row_count_p2 = 400000;
	let data = String::from("^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^")
		.chars()
		.map(|c| c == '^')
		.collect::<Vec<bool>>();

	let initial_count = data.iter().filter(|&x| !x).count() as u64;
	
	println!("Day 18 Part 1: {:?}", get_count(data.clone(), initial_count, row_count_p1));
	println!("Day 18 Part 2: {:?}", get_count(data, initial_count, row_count_p2));
}

fn get_count(input: Vec<bool>, initial_count: u64, rows: u64) -> u64 {
	let mut count = initial_count;
	let mut data = input;
	for _ in 0..rows - 1 {
		let result = generate_line(data);
		data = result.0;
		count += result.1;
	}
	count
}

fn generate_line(d: Vec<bool>) -> (Vec<bool>, u64) {
	let mut new_line = Vec::with_capacity(d.len());
	let mut count = 0;
	for i in 0..d.len() {
		let l = if i == 0 { false } else { d[i-1] };
		let r = if i == d.len() - 1 { false } else { d[i+1] };
		let t = is_trap(l, d[i], r);
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