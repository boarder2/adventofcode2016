mod input;
use regex::Regex;
use std::collections::VecDeque;

pub fn run() {
	let input = input::get_input();
	let mut scramble: VecDeque<char> = "abcdefgh".chars().collect::<VecDeque<char>>();
	let swap_pos = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
	let swap_letter = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
	let rotate_left = Regex::new(r"rotate left (\d+) step").unwrap();
	let rotate_right = Regex::new(r"rotate right (\d+) step").unwrap();
	let rotate_pos = Regex::new(r"rotate based on position of letter (\w)").unwrap();
	let reverse = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
	let move_pos = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

	for instruction in input.lines() {
		//println!("{:?} : \n{:?}", instruction, scramble.clone().into_iter().collect::<String>());
		if let Some(cap) = swap_pos.captures(instruction) {
			let a = cap.at(1).unwrap().parse().unwrap();
			let b = cap.at(2).unwrap().parse().unwrap();
			scramble.swap(a, b);
		} else if let Some(cap) = swap_letter.captures(instruction) {
			let ch_a = cap.at(1).unwrap().chars().next().unwrap();
			let ch_b = cap.at(2).unwrap().chars().next().unwrap();
			let a = scramble.iter().position(|&c| c == ch_a).unwrap();
			let b = scramble.iter().position(|&c| c == ch_b).unwrap();
			scramble.swap(a, b);
		} else if let Some(cap) = rotate_left.captures(instruction) {
			let c = cap.at(1).unwrap().parse().unwrap();
			for _ in 0..c {
				let t = scramble.pop_front().unwrap();
				scramble.push_back(t);
			}
		} else if let Some(cap) = rotate_right.captures(instruction) {
			let c = cap.at(1).unwrap().parse().unwrap();
			for _ in 0..c {
				let t = scramble.pop_back().unwrap();
				scramble.push_front(t);
			}
		} else if let Some(cap) = rotate_pos.captures(instruction) {
			let ch_a = cap.at(1).unwrap().chars().next().unwrap();
			let mut c = scramble.iter().position(|&c| c == ch_a).unwrap() + 1;
			if c >= 5 {
				c += 1;
			}
			for _ in 0..c {
				let t = scramble.pop_back().unwrap();
				scramble.push_front(t);
			}
		} else if let Some(cap) = reverse.captures(instruction) {
			let a = cap.at(1).unwrap().parse().unwrap();
			let b: usize = cap.at(2).unwrap().parse().unwrap();
			let mut reverse = scramble.split_off(a);
			let mut end = reverse.split_off((b + 1) - a);
			for ch in reverse.iter().rev() {
				scramble.push_back(*ch);
			}
			scramble.append(&mut end);
		} else if let Some(cap) = move_pos.captures(instruction) {
			let a = cap.at(1).unwrap().parse().unwrap();
			let b = cap.at(2).unwrap().parse().unwrap();
			let c = scramble.remove(a).unwrap();
			scramble.insert(b, c);
		}
		//println!("{:?}", scramble.clone().into_iter().collect::<String>());
	}

	println!("Day 21 Part 1: {:?}", scramble.into_iter().collect::<String>());
	println!("Day 21 Part 2: {:?}", "Not Implemented");
}