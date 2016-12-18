use crypto::digest::Digest;
use crypto::md5::Md5;
use day14::input;
use std::collections::HashMap;
use std::collections::LinkedList;

pub fn run() {
	let input = input::get_input();
	let input_bytes = input.as_bytes();
	let mut num_found = 0;
	let mut md5 = Md5::new();
	let mut possible_matches = HashMap::new();

	for i in 0..u64::max_value() {
		md5.input(input_bytes);
		md5.input(i.to_string().as_bytes());

		let hash = md5.result_str();

		if let Some(ch) = find_pattern(hash.to_owned(), 5) {
			//println!("Looking for {:?} in {:?}", ch, possible_matches);
			if let Some(ttl) = possible_matches.get(&ch) {
				if ttl > &0 {
					num_found += 1;
				} else {
					println!("Found {} in {:?} but it was too old", ch, possible_matches);
				}
			}
		}

		if let Some(ch) = find_pattern(hash.to_owned(), 3) {
			let e = possible_matches.entry(ch).or_insert(0);
			*e = 1000;
		}

		if num_found == 64 {
			println!("Day 14 Part 1: {}", i);
			break;
		}

		for pm in possible_matches.values_mut() {
			*pm -= 1;
		}
		md5.reset();
	}
}

fn find_pattern(s: String, num_required: usize) -> Option<char> {
	let mut section = LinkedList::new();

	'next_char: for ch in s.chars() {
		section.push_back(ch);
		if section.len() > num_required {
			section.pop_front();
		} else if section.len() != num_required {
			continue;
		}

		let comp = section.front().unwrap();
		for c in section.iter() {
			if c != comp {
				continue 'next_char;
			}
		}

		return Some(ch);
	}
	None
}

#[test]
fn pat() {
	let res = find_pattern("aaasdfjjjjjbaowerrrrjmmm222222".to_string(), 3);
	assert_eq!(res, Some('a'));
	let res = find_pattern("aaasdfjjjjjbaowerrrrjmmm222222".to_string(), 4);
	assert_eq!(res, Some('j'));
	let res = find_pattern("aaasdfjjjjjbaowerrrrjmmm222222".to_string(), 5);
	assert_eq!(res, Some('j'));
	let res = find_pattern("aaasdfjjjjjbaowerrrrjmmm222222".to_string(), 6);
	assert_eq!(res, Some('2'));
}