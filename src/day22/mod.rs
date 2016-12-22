mod input;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Node {
	x: u32,
	y: u32,
	size: u32,
	used: u32,
	avail: u32,
}

pub fn run() {
	let input = input::get_input();
	let node_rex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%")
		.unwrap();
	let mut pair_count = 0;
	let nodes: Vec<Node> = input.lines()
		.collect::<Vec<_>>()
		.iter()
		.filter(|l| node_rex.is_match(l))
		.map(|l| {
			let cap = node_rex.captures(l).unwrap();
			Node {
				x: cap.at(1).unwrap().parse().unwrap(),
				y: cap.at(2).unwrap().parse().unwrap(),
				size: cap.at(3).unwrap().parse().unwrap(),
				used: cap.at(4).unwrap().parse().unwrap(),
				avail: cap.at(5).unwrap().parse().unwrap(),
			}
		})
		.collect::<Vec<Node>>();

	let node_len = nodes.len();
	for i in 0..node_len {
		let n = nodes[i];
		if n.used == 0 {
			continue;
		}
		for j in 0..node_len {
			if j == i {
				continue;
			}
			if n.used <= nodes[j].avail {
				pair_count += 1;
			}
		}
	}

	println!("Day 22 Part 1: {:?}", pair_count);
	println!("Day 21 Part 2: {:?}", "Not Implemented");
}