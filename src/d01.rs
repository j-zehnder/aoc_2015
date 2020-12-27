#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '(' => Direction::Up,
            ')' => Direction::Down,
            _ => panic!("unknown char"),
        }
    }
}

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect()
}

#[aoc(day1, part1)]
pub fn part1(directions: &[Direction]) -> i32 {
    directions
        .iter()
        .map(|d| match d {
            Direction::Up => 1,
            Direction::Down => -1,
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(directions: &[Direction]) -> usize {
    let mut floor = 0;

    for (pos, d) in directions.iter().enumerate() {
        match d {
            Direction::Down => floor -= 1,
            Direction::Up => floor += 1,
        }
        if floor == -1 {
            return pos + 1; // zero-index to 1-indexed
        }
    }
    panic!("not found")
}
