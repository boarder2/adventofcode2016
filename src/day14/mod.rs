use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::HashMap;
// const SALT: &'static str = "abc";
const SALT: &'static str = "jlmsuwbz";

pub fn run() {
	println!("Day 14 Part 1: {}", find_index(0));
	println!("Day 14 Part 2: {}", find_index(2016));
}

fn find_index(rounds: usize) -> usize {
	let mut num_found = 0;
	let mut input_hashes = HashMap::new();

	for i in 0.. {
		let h = input_hashes.entry(i).or_insert_with(|| hash(i, rounds)).clone();
		if let Some(ch) = find_pattern(h.to_owned(), 3) {
			let ch5 = String::from_utf8(vec![ch as u8, ch as u8, ch as u8, ch as u8, ch as u8])
				.unwrap();
			for i_next in i + 1..i + 1001 {
				let h1 = input_hashes.entry(i_next).or_insert_with(|| hash(i_next, rounds));
				if h1.find(ch5.as_str()) != None {
					num_found += 1;
					if num_found >= 64 {
						return i as usize;
					}
					break;
				}
			}
		}
	}
	return 0;
}

fn hash(i: u64, rounds: usize) -> String {
	let mut md5 = Md5::new();
	md5.input_str(SALT);
	md5.input_str(&i.to_string());
	let mut hash = md5.result_str();
	for _ in 0..rounds {
		md5.reset();
		md5.input_str(&hash);
		hash = md5.result_str();
	}
	String::from(hash)
}

fn find_pattern(s: String, num_required: usize) -> Option<char> {
	for w in s.chars().collect::<Vec<char>>().windows(num_required) {
		let mut found = true;
		for i in 1..num_required {
			if w[0] != w[i] {
				found = false;
				break;
			}
		}
		if found {
			return Some(w[0]);
		}
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