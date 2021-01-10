#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Tile {
    Off,
    On,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Off,
            '#' => Tile::On,
            _ => panic!("unknown char"),
        }
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn step(tiles: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_tiles = tiles.to_vec();

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            let on_adjacent = count_on_adjacent(tiles, x, y);
            match tiles[y][x] {
                Tile::On => {
                    if on_adjacent != 2 && on_adjacent != 3 {
                        new_tiles[y][x] = Tile::Off;
                    }
                }
                Tile::Off => {
                    if on_adjacent == 3 {
                        new_tiles[y][x] = Tile::On;
                    }
                }
            }
        }
    }
    new_tiles
}

fn count_on_adjacent(tiles: &[Vec<Tile>], x: usize, y: usize) -> usize {
    let height = tiles.len();
    let width = tiles[0].len();

    let mut count = 0;

    for (dx, dy) in DIRECTIONS.iter() {
        let new_x = (x as isize) + dx;
        let new_y = (y as isize) + dy;

        if new_x >= 0
            && (new_x as usize) < width
            && new_y >= 0
            && (new_y as usize) < height
            && tiles[new_y as usize][new_x as usize] == Tile::On
        {
            count += 1;
        }
    }

    count
}

fn total_on_tiles(tiles: &[Vec<Tile>]) -> usize {
    let mut count = 0;
    for row in tiles {
        for tile in row {
            if tile == &Tile::On {
                count += 1;
            }
        }
    }
    count
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(tiles: &[Vec<Tile>]) -> usize {
    let mut my_tiles = tiles.to_vec();
    for _ in 0..100 {
        my_tiles = step(&my_tiles);
    }
    total_on_tiles(&my_tiles)
}

fn corners_on(tiles: &mut Vec<Vec<Tile>>) {
    tiles[0][0] = Tile::On;
    tiles[0][99] = Tile::On;
    tiles[99][0] = Tile::On;
    tiles[99][99] = Tile::On;
}

fn step_2(tiles: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_tiles = tiles.to_vec();
    corners_on(&mut new_tiles);

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            let on_adjacent = count_on_adjacent(tiles, x, y);
            match tiles[y][x] {
                Tile::On => {
                    if on_adjacent != 2 && on_adjacent != 3 {
                        new_tiles[y][x] = Tile::Off;
                    }
                }
                Tile::Off => {
                    if on_adjacent == 3 {
                        new_tiles[y][x] = Tile::On;
                    }
                }
            }
        }
    }
    corners_on(&mut new_tiles);
    new_tiles
}

#[aoc(day18, part2)]
pub fn part2(tiles: &[Vec<Tile>]) -> usize {
    let mut my_tiles = tiles.to_vec();
    for _ in 0..100 {
        my_tiles = step_2(&my_tiles);
    }
    total_on_tiles(&my_tiles)
}
