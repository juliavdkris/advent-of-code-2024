advent_of_code::solution!(1);

use itertools::Itertools;


pub fn part_one(input: &str) -> Option<u32> {
	let nums = input
		.split_ascii_whitespace()
		.map(|n| n.parse::<u32>().expect("Could not parse number"))
		.collect_vec();

	let col1 = nums
		.iter()
		.step_by(2)
		.sorted();

	let col2 = nums
		.iter()
		.skip(1)
		.step_by(2)
		.sorted();

	let sum = col1
		.zip(col2)
		.map(|(&n1, &n2)| n1.abs_diff(n2))
		.sum();

	Some(sum)
}


pub fn part_two(input: &str) -> Option<u32> {
	let nums = input
		.split_ascii_whitespace()
		.map(|n| n.parse::<u32>().expect("Could not parse number"))
		.collect_vec();

	let col1 = nums
		.iter()
		.step_by(2);

	let col2 = nums
		.iter()
		.skip(1)
		.step_by(2);

	let sum = col1
		.map(|n1| {
			let count = col2.clone().filter(|&n2| n1 == n2).count() as u32;
			n1 * count
		})
		.sum();

	Some(sum)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
