use day8::input;
use regex::Regex;
use std::collections::LinkedList;

pub fn run() {
	let input = input::get_input();
	let mut display = Vec::new();
	for _ in 0..6 {
		let mut ll = LinkedList::new();
		for _ in 0..50 {
			ll.push_back(false);
		}
		display.push(ll);
	}

	let rect_parser = Regex::new(r"rect (?P<x>\d*)x(?P<y>\d*)").unwrap();
	let rotate_parser =
		Regex::new(r"rotate \w* (?P<direction>\w)=(?P<index>\d*) by (?P<amount>\d*)").unwrap();

	for line in input.lines() {
		if let Some(captures) = rect_parser.captures(line) {
			let x = captures.name("x").unwrap().parse().unwrap();
			let y = captures.name("y").unwrap().parse().unwrap();
			for y in 0..y {
				let mut ll = display.get_mut(y).unwrap();
				let mut iter = ll.iter_mut();
				for _ in 0..x {
					let mut val = iter.next().unwrap();
					*val = true;
				}
			}
		} else if let Some(captures) = rotate_parser.captures(line) {
			let index = captures.name("index").unwrap().parse().unwrap();
			let amount = captures.name("amount").unwrap().parse().unwrap();
			if captures.name("direction").unwrap() == "y" {	
				for _ in 0..amount {
					let mut ll = display.get_mut(index).unwrap();
					let last = ll.pop_back().unwrap();
					ll.push_front(last);
				}
			} else {
				for _ in 0..amount {
					let last_value = *display[5].iter().nth(index).unwrap();
					let mut y = 6;
					for _ in 0..6 {
						y -= 1;
						let new_value;
						if y > 0 {
							new_value = *display[y - 1].iter().nth(index).unwrap();
						} else {
							new_value = last_value;
						}
						let mut current = display.get_mut(y).unwrap().iter_mut().nth(index).unwrap();
						*current = new_value;
					}

				}
			}
		} else {
			println!("found nothing");
		}
	}


	println!("Day 8 Part 1: {}", count_display(&display));
	print_display(&display);
}

fn print_display(display: &Vec<LinkedList<bool>>) {
	for y in display {
		for x in y.iter() {
			if *x {
				print!("{}", "#");
			} else {
				print!("{}", " ");
			}
		}
		println!("");
	}
}

fn count_display(display: &Vec<LinkedList<bool>>) -> u32 {
	let mut count = 0;
	for y in display {
		for x in y.iter() {
			if *x {
				count += 1;
			}
		}
	}
	count
}