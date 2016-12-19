pub mod part1;
pub mod part2;

const ELF_COUNT: usize = 3001330;
//const ELF_COUNT: usize = 5;

#[derive(Debug, Copy, Clone)]
struct Elf {
	number: u32,
	present_count: u32,
}