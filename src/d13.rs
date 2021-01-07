use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct SeatingRule {
    source: String,
    change: i32,
    adjacent: String,
}

impl SeatingRule {
    fn from_str(s: &str) -> Self {
        if let Ok((source, dir, chg, adjacent)) = scan_fmt!(
            s,
            "{} would {} {d} happiness units by sitting next to {}.",
            String,
            String,
            i32,
            String
        ) {
            let mut change = chg;
            if dir == "lose" {
                change = -chg;
            }
            return SeatingRule {
                source,
                change,
                adjacent,
            };
        }
        panic!("unknown format")
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Vec<SeatingRule> {
    input.lines().map(SeatingRule::from_str).collect()
}

#[aoc(day13, part1)]
pub fn part1(rules: &[SeatingRule]) -> i32 {
    let persons: HashSet<String> = rules.iter().map(|sr| sr.source.clone()).collect();

    let mut rule_map: HashMap<(String, String), i32> = HashMap::new();
    for rule in rules {
        rule_map.insert((rule.source.clone(), rule.adjacent.clone()), rule.change);
    }
    persons
        .iter()
        .permutations(persons.len())
        .map(|arr| calc_change(arr, &rule_map))
        .max()
        .unwrap()
}

fn calc_change(arrangement: Vec<&String>, rule_map: &HashMap<(String, String), i32>) -> i32 {
    let mut total_change = 0;
    for source_index in 0..arrangement.len() {
        let source = arrangement[source_index];

        let left_index;
        if source_index == 0 {
            left_index = arrangement.len() - 1;
        } else {
            left_index = source_index - 1;
        }

        let right_index;
        if source_index == arrangement.len() - 1 {
            right_index = 0;
        } else {
            right_index = source_index + 1;
        }

        let left = arrangement[left_index];
        let right = arrangement[right_index];

        total_change += rule_map.get(&(source.clone(), left.clone())).unwrap();
        total_change += rule_map.get(&(source.clone(), right.clone())).unwrap();
    }
    total_change
}

#[aoc(day13, part2)]
pub fn part2(rules: &[SeatingRule]) -> i32 {
    let myself = "MYSELF".to_string();

    let mut persons: HashSet<String> = rules.iter().map(|sr| sr.source.clone()).collect();

    let mut rule_map: HashMap<(String, String), i32> = HashMap::new();
    for rule in rules {
        rule_map.insert((rule.source.clone(), rule.adjacent.clone()), rule.change);
    }

    for person in &persons {
        rule_map.insert((myself.clone(), person.clone()), 0);
        rule_map.insert((person.clone(), myself.clone()), 0);
    }

    for rule in &rule_map {
        println!("{:?}", rule);
    }
    persons.insert(myself);

    persons
        .iter()
        .permutations(persons.len())
        .map(|arr| calc_change(arr, &rule_map))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d13_p1_t1() {
        let rules = parse_input(SAMPLE1);
        assert_eq!(330, part1(&rules));
    }

    const SAMPLE1: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
}
