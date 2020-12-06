/// Showcases genericity, "correct" / "idiomatic" way of parsing things in Rust
use crate::{
    error::AOCError,
    utils::grid::{Coordinates, Grid, GridParsingError},
};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TreeOrVoid {
    Void,
    Tree,
}

impl TryFrom<char> for TreeOrVoid {
    type Error = GridParsingError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(TreeOrVoid::Void),
            '#' => Ok(TreeOrVoid::Tree),
            c => Err(GridParsingError::UnknownChar(c)),
        }
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Result<Grid<TreeOrVoid>, AOCError> {
    Ok(Grid::<TreeOrVoid>::from_str(input)?)
}

trait RepeatingGridXAxis {
    type T;
    fn get_on_repeating_grid(&self, coords: Coordinates, max_coordinates: Coordinates) -> Self::T;
}

impl RepeatingGridXAxis for Grid<TreeOrVoid> {
    type T = TreeOrVoid;
    fn get_on_repeating_grid(&self, coords: Coordinates, max_coords: Coordinates) -> Self::T {
        let mapped_coordinates = Coordinates(coords.0 % (max_coords.0 + 1), coords.1);
        self.map.get(&mapped_coordinates).copied().unwrap()
    }
}

#[aoc(day3, part1)]
fn part_one(input: &Grid<TreeOrVoid>) -> usize {
    trees_for_slope(input, (3, 1))
}

#[aoc(day3, part2)]
fn part_two(input: &Grid<TreeOrVoid>) -> usize {
    trees_for_slope(input, (1, 1))
        * trees_for_slope(input, (3, 1))
        * trees_for_slope(input, (5, 1))
        * trees_for_slope(input, (7, 1))
        * trees_for_slope(input, (1, 2))
}

fn trees_for_slope(input: &Grid<TreeOrVoid>, slope: (i32, i32)) -> usize {
    let max_coords = Coordinates::from(input.get_max_coordinates());
    let mut curr_pos = Coordinates(0, 0);
    let mut count_trees = 0;
    while curr_pos.1 <= max_coords.1 {
        if input.get_on_repeating_grid(curr_pos, max_coords) == TreeOrVoid::Tree {
            count_trees += 1;
        }
        curr_pos.0 += slope.0;
        curr_pos.1 += slope.1;
    }

    count_trees
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const DUMMY_INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part_one() {
        let input = generator(DUMMY_INPUT).expect("Failed to parse Grid");
        assert_eq!(part_one(&input), 7);
    }

    #[test]
    fn test_part_two() {
        let input = generator(DUMMY_INPUT).expect("Failed to parse Grid");
        assert_eq!(part_two(&input), 336);
    }
}
