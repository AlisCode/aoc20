use std::collections::HashSet;

use crate::error::AOCError;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Result<Vec<i64>, AOCError> {
    Ok(input
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<Result<_, _>>()?)
}

#[aoc(day9, part1)]
fn part_one(input: &[i64]) -> i64 {
    solve_for_preamble_size(input, 25)
}

fn solve_for_preamble_size(input: &[i64], preamble_size: usize) -> i64 {
    let (index, _window) = input
        .windows(preamble_size + 1)
        .enumerate()
        .find(|(_index, window)| {
            let set: HashSet<i64> = window[0..=preamble_size - 1].iter().copied().collect();
            !window
                .iter()
                .any(|x| set.contains(&(window[preamble_size] - x)))
        })
        .expect("Failed to find answer");

    input[index + preamble_size]
}

#[aoc(day9, part2)]
fn part_two(input: &[i64]) -> i64 {
    let target = part_one(input);
    (2..)
        .filter_map(|window_size| {
            input
                .windows(window_size)
                .find(|w| w.iter().sum::<i64>() == target)
                .map(|w| {
                    let min = w.iter().min().expect("Failed to find min");
                    let max = w.iter().max().expect("Failed to find max");
                    min + max
                })
        })
        .next()
        .expect("Failed to find answer")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DUMMY_INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part_one() {
        let input = input_generator(DUMMY_INPUT).expect("Failed to parse input");
        assert_eq!(solve_for_preamble_size(&input, 5), 127);
    }
}
