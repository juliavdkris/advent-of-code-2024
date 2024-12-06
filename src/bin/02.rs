advent_of_code::solution!(2);

use itertools::Itertools;


pub fn part_one(input: &str) -> Option<u32> {
	let rows = input
		.lines()
		.map(|l| { l
			.split_ascii_whitespace()
			.map(|n| n.parse::<u32>().expect("Could not parse number"))
		});

	let count = rows
		.filter(|r| {
			let is_ordered = r.clone().is_sorted() || r.clone().rev().is_sorted();
			let is_gradual = r
				.clone()
				.tuple_windows()
				.map(|(n1, n2)| (1..=3).contains(&n1.abs_diff(n2)))
				.all(|b| b);
			is_ordered && is_gradual
		})
		.count() as u32;

	Some(count)
}


pub fn part_two(input: &str) -> Option<u32> {
	None
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
