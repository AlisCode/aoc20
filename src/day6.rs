use std::collections::HashSet;

#[aoc(day6, part1)]
fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut chars: HashSet<char> = HashSet::default();
            group.lines().for_each(|l| {
                l.chars().for_each(|c| {
                    chars.insert(c);
                })
            });
            chars.len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut chars: HashSet<char> = HashSet::default();
            group.lines().for_each(|l| {
                l.chars().for_each(|c| {
                    chars.insert(c);
                })
            });
            chars
                .into_iter()
                .filter(|c| group.lines().all(|l| l.contains(*c)))
                .count()
        })
        .sum()
}
