/// Parses the PasswordPolicy entries, using a Regex with Recap
/// because why not ?  
use crate::error::AOCError;
use recap::Recap;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, Recap, PartialEq, Eq)]
#[recap(
    regex = r#"(?P<range_min>(\d*))-(?P<range_max>(\d*)) (?P<wanted_char>([a-z])): (?P<passwd>([a-z]*))"#
)]
pub struct PasswordPolicy {
    pub range_min: usize,
    pub range_max: usize,
    pub wanted_char: String,
    pub passwd: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let count = self.passwd.matches(&self.wanted_char).count();
        (self.range_min..=self.range_max).contains(&count)
    }

    fn is_valid_part_two(&self) -> bool {
        let char_min = self.passwd.chars().skip(self.range_min - 1).next();
        let char_max = self.passwd.chars().skip(self.range_max - 1).next();
        let wanted_char = self
            .wanted_char
            .chars()
            .next()
            .expect("wanted_char is empty");
        (char_min == Some(wanted_char)) ^ (char_max == Some(wanted_char))
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Result<Vec<PasswordPolicy>, AOCError> {
    let output = input
        .lines()
        .map(|l| PasswordPolicy::from_str(l))
        .collect::<Result<Vec<PasswordPolicy>, recap::Error>>()?;
    Ok(output)
}

#[aoc(day2, part1)]
fn part_one(input: &[PasswordPolicy]) -> usize {
    input.iter().filter(|pp| pp.is_valid()).count()
}

#[aoc(day2, part2)]
fn part_two(input: &[PasswordPolicy]) -> usize {
    input.iter().filter(|pp| pp.is_valid_part_two()).count()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_password_policy() {
        let pp = PasswordPolicy::from_str("1-3 a: abcde").expect("Failed to parse");
        assert_eq!(
            pp,
            PasswordPolicy {
                range_min: 1,
                range_max: 3,
                wanted_char: "a".to_string(),
                passwd: "abcde".to_string(),
            }
        )
    }

    const DUMMY_INPUT: &'static str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part_one_should_work() {
        let input = generator(DUMMY_INPUT).expect("Failed to generate input");
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn part_two_should_work() {
        let input = generator(DUMMY_INPUT).expect("Failed to generate input");
        assert_eq!(part_two(&input), 1);
    }
}
