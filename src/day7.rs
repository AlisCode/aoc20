/// More non-trivial parsing!
/// First uses the naïve recursive approach.
/// Second uses a slightly optimized approach because recursion was exploding the stack
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::error::AOCError;

#[derive(Debug, PartialEq, Eq)]
struct BagRule {
    pub count: usize,
    pub contains: String,
}

#[derive(Debug)]
struct BagRules {
    pub color: String,
    pub rules: Vec<BagRule>,
}

impl FromStr for BagRule {
    type Err = std::num::ParseIntError;
    /// Format is  "<number> <color1> <color2> bag(s)" | "no other bag"
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "no other bags." {
            return Ok(BagRule {
                count: 0,
                contains: "nothing".to_string(),
            });
        }

        let input: Vec<&str> = input.split(" ").collect();
        let count: usize = input[0].parse()?;
        let contains = format!("{} {}", input[1], input[2]);

        Ok(BagRule { count, contains })
    }
}

impl FromStr for BagRules {
    type Err = std::num::ParseIntError;

    /// Format is "<color1> <color2> bags contain (<bag_rule>, )+"
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = input.split(" ").collect();
        let color = format!("{} {}", words[0], words[1]);

        let rules = words[4..]
            .join(" ")
            .split(", ")
            .map(BagRule::from_str)
            .collect::<Result<_, _>>()?;

        Ok(BagRules { color, rules })
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Result<Vec<BagRules>, AOCError> {
    Ok(input
        .lines()
        .map(BagRules::from_str)
        .collect::<Result<_, std::num::ParseIntError>>()?)
}

#[aoc(day7, part1)]
fn part_one(input: &[BagRules]) -> usize {
    let mut set = Default::default();
    part_one_recurse(input, "shiny gold", &mut set);
    set.len() - 1
}

fn part_one_recurse(rules: &[BagRules], color: &str, set: &mut HashSet<String>) {
    set.insert(color.to_string());
    for r in rules {
        if r.rules.iter().any(|rr| rr.contains == color) {
            if !set.contains(&r.color) {
                part_one_recurse(rules, &r.color, set)
            }
        }
    }
}

#[aoc(day7, part2)]
fn part_two(input: &[BagRules]) -> usize {
    let mut map: HashMap<String, usize> = Default::default();
    map.insert("nothing".to_string(), 0);
    let mut run = true;
    while run {
        run = false;
        for rules in input {
            if map.contains_key(&rules.color) {
                continue;
            }
            if rules.rules.iter().all(|rr| map.contains_key(&rr.contains)) {
                let sum: usize = rules
                    .rules
                    .iter()
                    .map(|rr| map[&rr.contains] * rr.count)
                    .sum();
                let entry = map.entry(rules.color.clone()).or_default();
                *entry = sum + 1;
                run = true;
            }
        }
    }

    map["shiny gold"] - 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DUMMY_INPUT: &'static str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn parse_bag_rules() {
        let bag_rules =
            BagRules::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .expect("Failed to parse");

        assert_eq!(bag_rules.color, "light red");
        assert_eq!(
            bag_rules.rules,
            vec![
                BagRule {
                    count: 1,
                    contains: "bright white".to_string(),
                },
                BagRule {
                    count: 2,
                    contains: "muted yellow".to_string(),
                },
            ]
        )
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(DUMMY_INPUT).expect("Failed while parsing");
        assert_eq!(part_one(&input), 4);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(DUMMY_INPUT).expect("Failed while parsing");
        assert_eq!(part_two(&input), 32);
    }
}
