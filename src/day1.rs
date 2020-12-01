/// Naively checking for all combinations  
use itertools::Itertools;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<u32> {
    let mut output: Vec<u32> = input.lines().map(|a| a.parse().unwrap()).collect();
    // Trick to make it faster with our kind of input: sorting.
    output.sort();
    output
}

fn find_for_combination(input: &[u32], combinations: usize) -> u32 {
    input
        .into_iter()
        .combinations(combinations)
        .find(|c| c.iter().copied().sum::<u32>() == 2020)
        .expect("Failed to find solution")
        .iter()
        .copied()
        .product()
}

#[aoc(day1, part1)]
fn part_one(input: &[u32]) -> u32 {
    find_for_combination(input, 2)
}

#[aoc(day1, part2)]
fn part_two(input: &[u32]) -> u32 {
    find_for_combination(input, 3)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DUMMY_INPUT: &'static str = "1721
979
366
299
675
1456";

    #[test]
    fn day_one_part_one() {
        let input = generator_input(DUMMY_INPUT);
        assert_eq!(part_one(&input), 514579);
    }

    #[test]
    fn day_one_part_two() {
        let input = generator_input(DUMMY_INPUT);
        assert_eq!(part_two(&input), 241861950);
    }
}
