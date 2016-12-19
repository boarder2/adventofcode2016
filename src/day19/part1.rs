use day19;

pub fn run() {
	//Hax - https://youtu.be/uCsD3ZGzMgE?t=601
	println!("Day 19 Part 1: {:?}", (day19::ELF_COUNT as u32 - 2u32.pow((day19::ELF_COUNT as f32).log2().trunc() as u32)) * 2 + 1);
}

// Naive implementation
// pub fn run() {
// 	let mut elves = Vec::with_capacity(day19::ELF_COUNT);

// 	for i in 0..day19::ELF_COUNT {
// 		let e = day19::Elf {
// 			number: (i + 1) as u32,
// 			present_count: 1,
// 		};
// 		elves.push(e);
// 	}

// 	println!("Day 19 Part 1: {:?}", reduce_to_one(elves));
// }

// fn reduce_to_one(e: Vec<day19::Elf>) -> u32 {
// 	let mut elves = e.clone();
// 	let len = elves.len();
// 	if len == 1 {
// 		return elves.get(0).unwrap().number;
// 	}
// 	for i in 0..len {
// 		{
// 			let current = elves.get(i).unwrap();
// 			if current.present_count == 0 {
// 				continue;
// 			}
// 		}
// 		let next_count;
// 		{
// 			let next = elves.get_mut(if i + 1 < len { i + 1 } else { 0 }).unwrap();
// 			next_count = next.present_count;
// 			next.present_count = 0;
// 		}
// 		let current = elves.get_mut(i).unwrap();
// 		current.present_count += next_count;
// 	}
// 	return reduce_to_one(elves.into_iter().filter(|x| x.present_count > 0).collect());
// }