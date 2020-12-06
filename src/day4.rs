/// Showcases newtype pattern.
/// Showcases usage of `?` (a bit of a hacky way to do early return)
/// The validation logic for part 2 is a bit of a pain...
///
/// This could also be done with the `nom` crate to showcase safe and maintainble parser combinators in Rust.
/// I have no time to do that right now though.
///
/// This part two is basically awful, but the logic behind it is easy so I got it to work first try and I'm not modifying this now.   
use std::collections::HashMap;

pub struct Passport(HashMap<String, String>);

impl From<&str> for Passport {
    fn from(input: &str) -> Self {
        let input = input.replace("\n", " ");
        let words = input.split(" ");
        let map: HashMap<String, String> = words
            .into_iter()
            .map(|w| {
                let w: Vec<&str> = w.split(":").into_iter().collect();
                (w[0].to_string(), w[1].to_string())
            })
            .collect();

        Passport(map)
    }
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.0.contains_key("byr")
            && self.0.contains_key("iyr")
            && self.0.contains_key("eyr")
            && self.0.contains_key("hgt")
            && self.0.contains_key("hcl")
            && self.0.contains_key("ecl")
            && self.0.contains_key("pid")
    }

    pub fn is_valid_part_two(&self) -> Option<bool> {
        let byr = self.0.get("byr")?;
        let byr: i32 = byr.parse::<i32>().ok()?;
        if byr < 1920 || byr > 2002 {
            return Some(false);
        }

        let iyr = self.0.get("iyr")?;
        let iyr: i32 = iyr.parse().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return Some(false);
        }

        let eyr = self.0.get("eyr")?;
        let eyr: i32 = eyr.parse().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return Some(false);
        }

        let hgt = self.0.get("hgt")?;
        let hgt_id: String = hgt.chars().rev().take(2).collect();
        match hgt_id.as_str() {
            "ni" => {
                let hgt = hgt.replace("in", "");
                let hgt: i32 = hgt.parse().ok()?;
                if hgt < 59 || hgt > 76 {
                    return Some(false);
                }
            }
            "mc" => {
                let hgt = hgt.replace("cm", "");
                let hgt: i32 = hgt.parse().ok()?;
                if hgt < 150 || hgt > 193 {
                    return Some(false);
                }
            }
            _ => {
                return Some(false);
            }
        }

        let hcl = self.0.get("hcl")?;
        if hcl.len() != 7 {
            return Some(false);
        }
        for (i, c) in hcl.char_indices() {
            match i {
                0 => {
                    if c != '#' {
                        return Some(false);
                    }
                }
                _ => {
                    let c = c as u8;
                    if !((48..=57).contains(&c) || (97..=102).contains(&c)) {
                        return Some(false);
                    }
                }
            }
        }

        let ecl = self.0.get("ecl")?;
        match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => {
                return Some(false);
            }
        }

        let pid = self.0.get("pid")?;
        if pid.len() != 9 {
            return Some(false);
        }
        let _pid: i32 = pid.parse().ok()?;

        Some(true)
    }
}

#[aoc(day4, part1)]
fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::from)
        .filter(Passport::is_valid)
        .count()
}

#[aoc(day4, part2)]
fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|inp| Passport::from(inp).is_valid_part_two())
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DUMMY_INPUT: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(DUMMY_INPUT), 2);
    }
}
