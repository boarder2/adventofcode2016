use day12::input;

pub fn run() {
	let input = input::get_input();
	let lines: Vec<&str> = input.lines().collect();
	let mut registers: [i32; 4] = [0; 4];
	registers[2] = 1;
	let mut inst_index = 0;
	while inst_index < lines.len() {
		let line = lines[inst_index];
		let instruction = line[0..3].to_string();
		let mut parts = line.split_whitespace();
		match instruction.as_ref() {
			"cpy" => {
				let value = get_register_value_or_integer_value(parts.nth(1).unwrap(), &registers);
				let register = get_register_index(parts.next().unwrap().chars().next().unwrap());
				registers[register] = value;
			}
			"inc" => {
				let register = get_register_index(parts.nth(1).unwrap().chars().next().unwrap());
				registers[register] += 1;
			}
			"dec" => {
				let register = get_register_index(parts.nth(1).unwrap().chars().next().unwrap());
				registers[register] -= 1;
			}
			"jnz" => {
				let do_jump;
				let part_a = parts.nth(1).unwrap();
				if is_register(part_a) {
					do_jump = registers[get_register_index(part_a.chars().next().unwrap())] > 0;
				} else {
					let v: i32 = part_a.parse().unwrap();
					do_jump = v > 0;
				}
				if do_jump {
 					let j: i32 = parts.next().unwrap().parse().unwrap();
					inst_index = (inst_index as i32 + j) as usize;
					continue;
				}
			}
			_ => println!("invalid instruction"),
		}
		inst_index += 1;
	}
	println!("Day 12 Part 2: {}", registers[0]);
}

fn get_register_value_or_integer_value(s: &str, registers: &[i32; 4]) -> i32 {
	if is_register(s) {
		registers[get_register_index(s.chars().next().unwrap())]
	} else {
		s.parse().unwrap()
	}
}

fn is_register(s: &str) -> bool {
	let b = s.chars().next().unwrap() as u8;
	b >= 97 && b <= 100
}

fn get_register_index(ch: char) -> usize {
	(ch as u8 - 97) as usize
}