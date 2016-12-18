use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::LinkedList;

pub fn run() {
	//let input_bytes = "jlmsuwbz".as_bytes();
	let input_bytes = "abc".as_bytes();
	let mut num_found = 0;
	let mut md5 = Md5::new();

	for i in 0..u64::max_value() {
		md5.input(input_bytes);
		md5.input(i.to_string().as_bytes());

		let hash = md5.result_str();

		if let Some(ch) = find_pattern(hash.to_owned(), 3) {
			let mut ch5 = String::with_capacity(5);
			for _ in 0..5 {
				ch5.push(ch);
			}
			for i_next in i + 1..i + 1001 {
				md5.reset();
				md5.input(input_bytes);
				md5.input(i_next.to_string().as_bytes());
				let h1 = md5.result_str();
				if h1.find(ch5.as_str()) != None {
					num_found += 1;
					if num_found >= 64 {
						println!("Day 14 Part 1: {}", i);
						return;
					}
					break;
				}
			}
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