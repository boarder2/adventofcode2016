mod input;

use regex::Regex;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Bot {
	number: u32,
	high: u32,
	low: u32,
	high_is_output: bool,
	low_is_output: bool,
	chips: Vec<u32>,
}

pub fn run() {
	let input = input::get_input();
	let mut value_instructions = Vec::new();
	let mut bots = HashMap::new();
	let mut outputs = HashMap::new();
	
	let bot_regex =
		Regex::new(r"bot (?P<bot>\d+) gives low to (?P<lowtype>bot|output) (?P<low>\d+) and high to (?P<hightype>bot|output) (?P<high>\d+)")
			.unwrap();
	let value_regex = Regex::new(r"value (?P<value>\d+) goes to bot (?P<bot>\d+)").unwrap();

	for line in input.lines() {
		if let Some(capture) = bot_regex.captures(line) {
			let bot: u32 = capture.name("bot").unwrap().parse().unwrap();
			let low = capture.name("low").unwrap().parse().unwrap();
			let high = capture.name("high").unwrap().parse().unwrap();
			let high_is_output = capture.name("hightype").unwrap() == "output";
			let low_is_output = capture.name("lowtype").unwrap() == "output";
			let b = Bot {
				number: bot,
				low: low,
				high: high,
				low_is_output: low_is_output,
				high_is_output: high_is_output,
				chips: Vec::new(),
			};
			if low_is_output {
				 if !outputs.contains_key(&low) {
					 outputs.insert(low, Vec::new());
				 }
			}
			if high_is_output {
				 if !outputs.contains_key(&high) {
					 outputs.insert(high, Vec::new());
				 }
			}
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

	let mut continue_processing = true;
	while continue_processing {
		let mut bots_to_process = Vec::new();
		continue_processing = false;
		for b in bots.values() {
			if b.chips.len() >= 2 {
				bots_to_process.push(b.number);
				continue_processing = true;
			}
		}
		for b in bots_to_process {
			let mut bot;
			{
				let mut local_bot = bots.get_mut(&b).unwrap();
				bot = Bot {
					number: local_bot.number,
					high: local_bot.high,
					low: local_bot.low,
					high_is_output: local_bot.high_is_output,
					low_is_output: local_bot.low_is_output,
					chips: Vec::new()
				};
				while let Some(chip) = local_bot.chips.pop() {
					 bot.chips.push(chip);
				}
			}
			
			if bot.chips.contains(&61) && bot.chips.contains(&17) {
				println!("Day 10 Part 1: {}", b);
			}

			{
				if bot.high_is_output {
					let output_dest = outputs.get_mut(&bot.high).unwrap();
					output_dest.push(cmp::max(bot.chips[0], bot.chips[1]));
				} else {
					let high_dest = bots.get_mut(&bot.high).unwrap();
					high_dest.chips.push(cmp::max(bot.chips[0], bot.chips[1]));
				}
			}
			{
				if bot.low_is_output {
					let output_dest = outputs.get_mut(&bot.low).unwrap();
					output_dest.push(cmp::min(bot.chips[0], bot.chips[1]));
				} else {
					let low_dest = bots.get_mut(&bot.low).unwrap();
					low_dest.chips.push(cmp::min(bot.chips[0], bot.chips[1]));
				}
			}
		}
	}
	println!("Day 10 Part 2: {}", outputs.get(&0).unwrap()[0] * outputs.get(&1).unwrap()[0] * outputs.get(&2).unwrap()[0]);
}