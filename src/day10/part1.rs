use day10::input;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Bot {
	 high: usize,
	 low: usize,
	 chips: Vec<u32>,
}

pub fn run() {
	let input = input::get_input();
	let mut value_instructions = Vec::new();
	let mut bots = HashMap::new();
	let bot_regex = Regex::new(r"bot (?P<bot>\d+) gives low to bot (?P<low>\d+) and high to bot (?P<high>\d+)").unwrap();
	let value_regex = Regex::new(r"value (?P<value>\d+) goes to bot (?P<bot>\d+)").unwrap();

	for line in input.lines() {
		 if let Some(capture) = bot_regex.captures(line) {
			 let bot: u32 = capture.name("bot").unwrap().parse().unwrap();
			 let low = capture.name("low").unwrap().parse().unwrap();
			 let high = capture.name("high").unwrap().parse().unwrap();
			 let b = Bot { low: low, high: high, chips: Vec::new() };
			 bots.insert(bot, b);
		 } else {
			  value_instructions.push(line);
		 }
	}

	for line in value_instructions {
		if let Some(capture) = value_regex.captures(line) {
			 let bot: u32 = capture.name("bot").unwrap().parse().unwrap();
			 let value = capture.name("value").unwrap().parse().unwrap();
			 if let Some(b) = bots.get_mut(&bot) {
				  b.chips.push(value);
			 }
		}
	}

	println!("{:?}", bots);

	for b in bots.values() {
		if b.chips.len() >= 2 {
			println!("{:?}", b);
		}
	}


	println!("Day 10 Part 1: {}", "Not implemented");
}