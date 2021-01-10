use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day17, part1)]
pub fn part1(buckets: &[i32]) -> usize {
    let mut combinations: usize = 0;
    for k in 1..=buckets.len() {
        for comb in buckets.iter().combinations(k) {
            if comb.iter().map(|i| **i).sum::<i32>() == 150 {
                combinations += 1;
            }
        }
    }
    combinations
}

#[aoc(day17, part2)]
pub fn part2(buckets: &[i32]) -> usize {
    let mut combinations: HashMap<usize, usize> = HashMap::new();
    for k in 1..=buckets.len() {
        let mut k_combinations: usize = 0;
        for comb in buckets.iter().combinations(k) {
            if comb.iter().map(|i| **i).sum::<i32>() == 150 {
                k_combinations += 1;
            }
        }
        if k_combinations > 0 {
            combinations.insert(k, k_combinations);
        }
    }

    let min_k = combinations.keys().min().unwrap();

    *combinations.get(min_k).unwrap()
}
