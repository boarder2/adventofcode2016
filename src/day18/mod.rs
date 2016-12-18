pub fn run() {
	let row_count_p1 = 40;
	let row_count_p2 = 400000;
	let input = String::from("^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^")
	//let input = String::from(".^^.^.^^^^")
		.chars()
		.map(|c| c == '^')
		.collect::<Vec<bool>>();
	let mut map = Vec::new();

	// println!("{:?}",
	//          input.clone().into_iter().map(|x| if x { "^" } else { "." }).collect::<String>());
	map.push(input.clone());

	for i in 0..row_count_p1 - 1 {
		let new_line = generate_line(map.get(i).unwrap().clone());
		// println!("{:?}",
		//          new_line.clone().into_iter().map(|x| if x { "^" } else { "." }).collect::<String>());
		map.push(new_line);
	}

	println!("Day 18 Part 1: {:?}", get_safe_count(map));
	
	let mut map = Vec::new();
	map.clear();

	// println!("{:?}",
	//          input.clone().into_iter().map(|x| if x { "^" } else { "." }).collect::<String>());
	map.push(input);

	for i in 0..row_count_p2 - 1 {
		let new_line = generate_line(map.get(i).unwrap().clone());
		// println!("{:?}",
		//          new_line.clone().into_iter().map(|x| if x { "^" } else { "." }).collect::<String>());
		map.push(new_line);
	}


	println!("Day 18 Part 2: {:?}", get_safe_count(map));
}

fn get_safe_count(map: Vec<Vec<bool>>) -> u32 {
	map.iter()
        .map(|x| x
            .iter()
            .map(|&y| if y { 0 } else { 1 })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>())
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

fn generate_line(d: Vec<bool>) -> Vec<bool> {
	let mut new_line = Vec::with_capacity(d.len());
	for i in 0..d.len() {
		if i == 0 {
			new_line.push(is_trap(false, d[i], d[i + 1]));
		} else if i == d.len() - 1 {
			new_line.push(is_trap(d[i - 1], d[i], false));
		} else {
			new_line.push(is_trap(d[i - 1], d[i], d[i + 1]));
		}
	}
	new_line
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