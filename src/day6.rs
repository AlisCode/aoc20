/// More iterators showcase.
/// Two solutions to part one : interestingly enough, folding seems to
/// be benefiting from better optimization ?
use std::collections::HashSet;

#[aoc(day6, part1)]
fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|l| l.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part1, fold)]
fn part_one_fold(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(HashSet::<char>::default(), |mut acc, l| {
                    l.chars().for_each(|c| {
                        acc.insert(c);
                    });
                    acc
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let set: HashSet<char> = group.lines().flat_map(|l| l.chars()).collect();
            set.into_iter()
                .filter(|c| group.lines().all(|l| l.contains(*c)))
                .count()
        })
        .sum()
}

// Yea, we could also fold and use a hashmap for part2 instead of double-iterating. No time.
