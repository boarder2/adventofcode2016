use day4::input;
use regex::Regex;

pub fn run() {
	let input = input::get_input();
	let lines = input.lines();
	let reg = Regex::new(r"(?P<name>[a-z-]+)-(?P<sector>\d+)\[[a-z]+\]").unwrap();

	for line in lines {
		let capture = reg.captures(line).unwrap();
		let sector_id = capture.name("sector").unwrap().parse().unwrap();
		let encrypted_name = &capture.name("name").unwrap().replace("-", "");
		let decrypted_name = decrypt_name(encrypted_name, sector_id);
		//println!("{}", decrypted_name);
		if decrypted_name == "northpoleobjectstorage" {
			println!("Day 4 Part 2: {}", sector_id);
			break;
		}
	}
}

fn decrypt_name(encrytped_name: &str, sector_id: i32) -> String {
	let mut unencrypted = String::with_capacity(encrytped_name.len());
	for ch in encrytped_name.bytes() {
		let ch_i = ch as i32;
		let new_char = ((((ch_i - 97) + sector_id) % 26) + 97) as u8;
		unencrypted.push(new_char as char);
	}
	unencrypted
}