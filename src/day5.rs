struct Seat {
    pub row: u8,
    pub column: u8,
}

impl Seat {
    pub fn id(&self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }
}

impl From<&str> for Seat {
    fn from(input: &str) -> Seat {
        let row = input[0..7]
            .chars()
            .fold(0, |acc, c| if c == 'B' { (acc << 1) + 1 } else { acc << 1 });
        let column = input[7..=9]
            .chars()
            .fold(0, |acc, c| if c == 'R' { (acc << 1) + 1 } else { acc << 1 });

        Seat { row, column }
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Seat> {
    input.lines().map(Seat::from).collect()
}

#[aoc(day5, part1)]
fn part_one(input: &[Seat]) -> u32 {
    input
        .iter()
        .map(Seat::id)
        .max()
        .expect("Failed to find max")
}

#[aoc(day5, part2)]
fn part_two(input: &[Seat]) -> u32 {
    let mut seat_ids: Vec<u32> = input.iter().map(Seat::id).collect();
    seat_ids.sort();
    let min = seat_ids[0];
    let max = seat_ids[seat_ids.len() - 1];
    (min..max)
        .find(|id| seat_ids.binary_search(id).is_err())
        .expect("Failed to find empty seat")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_seat() {
        let test_seat = Seat::from("BFFFBBFRRR");
        assert_eq!(test_seat.row, 70);
        assert_eq!(test_seat.column, 7);
        assert_eq!(test_seat.id(), 567);
    }
}
