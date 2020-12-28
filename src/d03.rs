use std::collections::HashSet;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("unknown char"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect()
}

#[aoc(day3, part1)]
pub fn part1(directions: &[Direction]) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(Coord { x: 0, y: 0 });

    let mut x = 0;
    let mut y = 0;

    for direction in directions {
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        visited.insert(Coord { x, y });
    }

    visited.len()
}

#[aoc(day3, part2)]
pub fn part2(directions: &[Direction]) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(Coord { x: 0, y: 0 });

    // santa
    let mut x = 0;
    let mut y = 0;

    directions.iter().step_by(2).for_each(|d| {
        match d {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        visited.insert(Coord { x, y });
    });

    // robot
    x = 0;
    y = 0;
    directions.iter().skip(1).step_by(2).for_each(|d| {
        match d {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        visited.insert(Coord { x, y });
    });

    visited.len()
}
