/// Some AOC problems require a 2-dimensional grid.
/// This is meant to automate the parsing of such a grid.
use std::{collections::HashMap, convert::TryFrom, str::FromStr};
use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates(pub i32, pub i32);

impl From<(i32, i32)> for Coordinates {
    fn from(t: (i32, i32)) -> Self {
        Coordinates(t.0, t.1)
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub map: HashMap<Coordinates, T>,
}

#[derive(Debug, Error)]
pub enum GridParsingError {
    #[error("Unknown char _0")]
    UnknownChar(char),
}

impl<T: TryFrom<char, Error = GridParsingError>> FromStr for Grid<T> {
    type Err = GridParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::default();
        let mut y = 0;
        for line in s.lines() {
            for (x, c) in line.char_indices() {
                let t = T::try_from(c)?;
                map.insert(Coordinates(x as i32, y), t);
            }
            y += 1;
        }
        Ok(Grid { map })
    }
}

impl<T> Grid<T> {
    pub fn get_max_coordinates(&self) -> (i32, i32) {
        let coords = self
            .map
            .keys()
            .fold((i32::MIN, i32::MIN), |(x, y), coords| {
                (x.max(coords.0), y.max(coords.1))
            });
        (coords.0, coords.1)
    }
}
