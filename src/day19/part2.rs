//use day19;

//Naive implementation - stack overflows.
// pub fn run() {
// 	let mut elves = Vec::with_capacity(day19::ELF_COUNT);

// 	for i in 0..day19::ELF_COUNT {
// 		let e = day19::Elf {
// 			number: (i + 1) as u32,
// 			present_count: 1,
// 		};
// 		elves.push(e);
// 	}

// 	println!("Day 19 Part 2: {:?}", reduce_to_one(elves, 0));
// }

// fn reduce_to_one(mut elves: Vec<day19::Elf>, index: usize) -> u32 {
// 	let len = elves.len();
// 	if len == 1 {
// 		return elves.get(0).unwrap().number;
// 	}
// 	let half = (len / 2) as usize;
// 	let donor_index = if half + index > len - 1 {
// 		(half + index) - len
// 	} else {
// 		half + index
// 	};
// 	let donor_count;
// 	{
// 		let donor = elves.get_mut(donor_index).unwrap();
// 		donor_count = donor.present_count;
// 		donor.present_count = 0;
// 	}
// 	{
// 		let taker = elves.get_mut(index).unwrap();
// 		taker.present_count += donor_count;
// 	}
// 	return reduce_to_one(elves.into_iter().filter(|x| x.present_count > 0).collect(),
// 	                     if index + 1 > len - 1 { 0 } else { index + 1 });
// }