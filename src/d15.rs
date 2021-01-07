use itertools::Itertools;
use std::cmp::max;

#[derive(Debug)]
pub struct CookieData {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl CookieData {
    fn from_str(s: &str) -> Self {
        if let Ok((name, capacity, durability, flavor, texture, calories)) = scan_fmt!(
            s,
            "{}: capacity {d}, durability {d}, flavor {d}, texture {d}, calories {d}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32
        ) {
            return CookieData {
                name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            };
        }
        panic!("invalid input format")
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<CookieData> {
    input.lines().map(CookieData::from_str).collect()
}

#[aoc(day15, part1)]
pub fn part1(cookies: &[CookieData]) -> i32 {
    cookies
        .iter()
        .combinations_with_replacement(100)
        .map(|recipe| score_cookie(recipe))
        .max()
        .unwrap()
}

#[aoc(day15, part2)]
pub fn part2(cookies: &[CookieData]) -> i32 {
    cookies
        .iter()
        .combinations_with_replacement(100)
        .map(|recipe| score_cookie_2(recipe))
        .max()
        .unwrap()
}

fn score_cookie(ingredients: Vec<&CookieData>) -> i32 {
    let capacity: i32 = ingredients.iter().map(|cd| cd.capacity).sum();
    let durability: i32 = ingredients.iter().map(|cd| cd.durability).sum();
    let flavor: i32 = ingredients.iter().map(|cd| cd.flavor).sum();
    let texture: i32 = ingredients.iter().map(|cd| cd.texture).sum();
    [capacity, durability, flavor, texture]
        .iter()
        .map(|&x| max(0, x))
        .product()
}

fn score_cookie_2(ingredients: Vec<&CookieData>) -> i32 {
    if ingredients.iter().map(|cd| cd.calories).sum::<i32>() == 500 {
        return score_cookie(ingredients);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15_p1_t1() {
        let data = parse_input(SAMPLE1);
        assert_eq!(62842880, part1(&data));
    }

    #[test]
    fn d15_p2_t1() {
        let data = parse_input(SAMPLE1);
        assert_eq!(57600000, part2(&data));
    }

    const SAMPLE1: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
}
